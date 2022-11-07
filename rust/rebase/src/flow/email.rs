use crate::{
    content::email::Email as Ctnt,
    flow::response::PostResponse,
    proof::email::Email as Prf,
    statement::email::Email as Stmt,
    types::{
        error::FlowError,
        types::{Flow, Instructions, Issuer, Proof, Statement, Subject},
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
use url::Url;

#[derive(Clone, Deserialize, Serialize)]
pub struct SendGridBasic {
    api_key: String,
    from_addr: String,
    from_name: String,
    // This is checked for a negative value and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    max_elapsed_minutes: i64,
    subject_name: String,
}

impl SendGridBasic {
    async fn body<I: Issuer>(&self, stmt: &Stmt, issuer: &I) -> Result<String, FlowError> {
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let statement = format!("{}:::{}", stmt.generate_statement()?, now);
        let challenge = issuer.sign(&statement).await?;
        Ok(format!("Please paste the following into the challenge input on the witness page used to generate this email:\n\nChallenge:\n\n{}:::{}", challenge, now))
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
impl Flow<Ctnt, Stmt, Prf, PostResponse> for SendGridBasic {
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
    ) -> Result<PostResponse, FlowError> {
        let statement = stmt.generate_statement()?;
        let b = self.body(stmt, issuer).await?;
        let s = self.subject(stmt).await?;
        let req = json!({
            "personalizations": [{
                    "to": [
                        {
                            "email": stmt.email,
                            // TODO: Add name?
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

        let u = Url::parse("https://api.sendgrid.com/v3/mail/send").map_err(|e| {
            FlowError::BadLookup(format!("Failed to parse email API Url: {}", e.to_string()))
        })?;

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
            .map_err(|e| {
                FlowError::BadLookup(format!("Could not send email: {}", e.to_string()))
            })?;

        Ok(PostResponse {
            statement,
            // TODO: REMOVE WHEN DOING BREAKING CHANGES
            delimitor: "\n\n".to_owned(),
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, issuer: &I) -> Result<Ctnt, FlowError> {
        if self.max_elapsed_minutes <= 0 {
            return Err(FlowError::Validation(
                "Max elapsed minutes must be set to a number greater than 0".to_string(),
            ));
        }

        let now = Utc::now();
        let then = DateTime::parse_from_rfc3339(&proof.timestamp)
            .map_err(|e| FlowError::Validation(e.to_string()))?;

        if now - Duration::minutes(self.max_elapsed_minutes) > then {
            return Err(FlowError::Validation(format!(
                "Validation window has expired"
            )));
        }

        let t = format!(
            "{}:::{}",
            proof.statement.generate_statement()?,
            proof.timestamp
        );

        issuer.valid_signature(&t, &proof.auth).await?;

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
