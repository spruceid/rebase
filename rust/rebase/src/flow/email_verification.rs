use crate::{
    content::email_verification::EmailVerificationContent as Ctnt,
    proof::email_verification::EmailVerificationProof as Prf,
    statement::email_verification::EmailVerificationStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        error::FlowError,
    },
};

use async_trait::async_trait;
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SendGridBasicFlow {
    pub api_key: String,
    pub challenge_delimiter: String,
    pub from_addr: String,
    pub from_name: String,
    // This is checked for a negative value or 0 and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    pub max_elapsed_minutes: i64,
    pub subject_name: String,
}

impl SendGridBasicFlow {
    async fn body<I: Issuer>(&self, stmt: &Stmt, issuer: &I) -> Result<String, FlowError> {
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let statement = format!(
            "{}{}{}",
            stmt.generate_statement()?,
            &self.challenge_delimiter,
            now
        );
        let challenge = issuer.sign(&statement).await?;
        Ok(format!("Please paste the following into the challenge input on the witness page used to generate this email:\n\n{}:::{}", challenge, now))
    }

    async fn subject(&self, stmt: &Stmt) -> Result<String, FlowError> {
        Ok(format!(
            "Verifying ownership of {} {} for {}",
            stmt.subject.statement_title()?,
            stmt.subject.display_id()?,
            self.subject_name
        ))
    }
}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf> for SendGridBasicFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter the email addres you wish to prove the ownership of.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your email address and additional information.".to_string(),
            witness: "Find the email sent from the witness and copy the code and challenge into the respective form fields.".to_string(),
            witness_schema: schema_for!(Prf),
        })
    }

    async fn statement<I: Issuer>(
        &self,
        stmt: &Stmt,
        issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        let statement = stmt.generate_statement()?;
        let b = self.body(stmt, issuer).await?;
        let s = self.subject(stmt).await?;
        let req = json!({
            "personalizations": [{
                    "to": [
                        {
                            // TODO: Add name?
                            "email": stmt.email,
                        }
                    ],
                    "subject": s
            }],
            "content": [
                {
                    "type": "text/plain",
                    "value": b,
                }
            ],
            "from": {
                "email": self.from_addr,
                "name": self.from_name,
            }
        });

        let u = Url::parse("https://api.sendgrid.com/v3/mail/send")
            .map_err(|e| FlowError::BadLookup(format!("Failed to parse email API Url: {}", e)))?;

        let key_header: HeaderValue =
            format!("Bearer {}", &self.api_key).parse().map_err(|_| {
                FlowError::BadLookup("Failed to generate authorization header".to_string())
            })?;

        let content_header: HeaderValue = "application/json".parse().map_err(|_| {
            FlowError::BadLookup("Failed to generate authorization header".to_string())
        })?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, key_header);
        headers.insert(CONTENT_TYPE, content_header);

        let client = Client::new();

        client
            .post(u)
            .headers(headers)
            .json(&req)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(format!("Could not send email: {}", e)))?;

        Ok(StatementResponse {
            statement,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, issuer: &I) -> Result<Ctnt, FlowError> {
        if self.max_elapsed_minutes <= 0 {
            return Err(FlowError::Validation(
                "Max elapsed minutes must be set to a number greater than 0".to_string(),
            ));
        }

        let challenge_vec: Vec<&str> = proof.challenge.split(&self.challenge_delimiter).collect();
        if challenge_vec.len() != 2 {
            return Err(FlowError::Validation(
                "Challenge in unexpected format".to_string(),
            ));
        }

        let ch = challenge_vec[0];
        let ts = challenge_vec[1];

        let now = Utc::now();
        let then =
            DateTime::parse_from_rfc3339(ts).map_err(|e| FlowError::Validation(e.to_string()))?;

        if now - Duration::minutes(self.max_elapsed_minutes) > then {
            return Err(FlowError::Validation(
                "Validation window has expired".to_string(),
            ));
        }

        let t = format!(
            "{}{}{}",
            proof.statement.generate_statement()?,
            &self.challenge_delimiter,
            ts
        );

        issuer.valid_signature(&t, ch).await?;

        let s = proof.statement.generate_statement()?;
        proof
            .statement
            .subject
            .valid_signature(&s, &proof.signature)
            .await?;

        Ok(proof.to_content(&s, &proof.signature)?)
    }
}

/* TODO: Add automated tests. */

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use super::*;
    use crate::{
        test_util::util::test_did_keypair,
        types::defs::{Issuer, Statement},
    };

    #[tokio::test]
    async fn mock_email() {
        // get witness' issuer:
        // NOTE: A working issuer is required for these tests.
        let (_, i) = test_did_keypair().await.unwrap();

        // Test it works as expected.

        // get subjects keypair
        let (subj1, iss1) = test_did_keypair().await.unwrap();
        let ver_stmt1 = Stmt {
            subject: subj1.clone(),
            email: "example@example.com".to_string(),
        };

        // Because the lookup happens at the Statement step, we can use a real flow struct, unlike other flows.
        let flow = SendGridBasicFlow {
            api_key: "unimplemented".to_string(),
            challenge_delimiter: ":::".to_string(),
            from_addr: "unimplemented".to_string(),
            from_name: "unimplemented".to_string(),
            subject_name: "unimplemented".to_string(),
            max_elapsed_minutes: 10,
        };

        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let s = format!(
            "{}{}{}",
            ver_stmt1.generate_statement().unwrap(),
            ":::",
            now
        );
        let witness_sig = i.sign(&s).await.unwrap();
        let challenge = format!("{}:::{}", witness_sig, now);

        let s1 = &ver_stmt1.generate_statement().unwrap();
        let sig1 = iss1.sign(s1).await.unwrap();

        let ver_proof1 = Prf {
            challenge: challenge.clone(),
            statement: ver_stmt1.clone(),
            signature: sig1.clone(),
        };

        flow.jwt(&ver_proof1, &i).await.unwrap();

        // Test it detects a bad signature.

        let (_, iss2) = test_did_keypair().await.unwrap();
        let bad_sig = iss2.sign(s1).await.unwrap();

        let bad_proof1 = Prf {
            challenge: challenge.clone(),
            statement: ver_stmt1.clone(),
            signature: bad_sig,
        };

        match flow.jwt(&bad_proof1, &i).await {
            Err(_) => {}
            Ok(_) => panic!("Accepted bad signature"),
        }

        // Test it detects a bad challenge.

        // We'll use this in the next test, but it will be useful now as a mismatched challenge.
        let t = Utc::now();
        let earlier = t
            .sub(Duration::minutes(30))
            .to_rfc3339_opts(SecondsFormat::Millis, true);

        let bad_s = format!(
            "{}{}{}",
            ver_stmt1.generate_statement().unwrap(),
            ":::",
            earlier
        );
        let expired_witness_sig = i.sign(&bad_s).await.unwrap();

        // NOTE: Here we use "now" so that the datetime checking isn't
        // triggered, and the challenge's cryptographic integrity is checked.
        let bad_challenge = format!("{}:::{}", expired_witness_sig, now);

        let bad_proof2 = Prf {
            challenge: bad_challenge,
            statement: ver_stmt1.clone(),
            signature: sig1.clone(),
        };
        match flow.jwt(&bad_proof2, &i).await {
            Err(_) => {}
            Ok(_) => panic!("Accepted bad challenge"),
        }

        // Test it detects an expired challenge
        let expired_challenge = format!("{}:::{}", expired_witness_sig, earlier);
        let bad_proof3 = Prf {
            challenge: expired_challenge,
            statement: ver_stmt1.clone(),
            signature: sig1.clone(),
        };
        match flow.jwt(&bad_proof3, &i).await {
            Err(_) => {}
            Ok(_) => panic!("Accepted expired challenge"),
        }
    }
}
