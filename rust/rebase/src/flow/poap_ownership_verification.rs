use crate::{
    content::poap_ownership_verification::PoapOwnershipVerificationContent as Ctnt,
    proof::poap_ownership_verification::PoapOwnershipVerificationProof as Prf,
    statement::poap_ownership_verification::PoapOwnershipVerificationStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        enums::subject::{Pkh, Subjects},
        error::FlowError,
    },
};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PoapOwnershipVerificationFlow {
    pub api_key: String,
    pub challenge_delimiter: String,
    // The amount of time that can pass before the witness
    // wants a new flow initiated. In demo, set to 15 mins.
    // This is checked for a negative value or 0 and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    pub max_elapsed_minutes: i64,
}

impl PoapOwnershipVerificationFlow {
    // This makes sure the timestamps the client supplies make sense and are
    // with in the limits of configured expration and that the max elapsed
    // minutes are greater than 0.
    pub fn sanity_check(&self, timestamp: &str) -> Result<(), FlowError> {
        if self.max_elapsed_minutes <= 0 {
            return Err(FlowError::Validation(
                "Max elapsed minutes must be set to a number greater than 0".to_string(),
            ));
        }

        let now = Utc::now();
        let then = DateTime::parse_from_rfc3339(timestamp)
            .map_err(|e| FlowError::Validation(e.to_string()))?;

        if then > now {
            return Err(FlowError::Validation(
                "Timestamp provided comes from the future".to_string(),
            ));
        }

        if now - Duration::minutes(self.max_elapsed_minutes) > then {
            return Err(FlowError::Validation(
                "Validation window has expired".to_string(),
            ));
        };
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
struct PoapResEntry {
    event: PoapEventEntry,
    #[serde(rename = "tokenId")]
    token_id: String,
    owner: String,
    chain: String,
    // NOTE: This date is in the format "YYYY-MM-DD HH-mm-ss"
    created: String,
}

#[derive(Deserialize, Serialize)]
struct PoapEventEntry {
    id: i64,
    fancy_id: String,
    name: String,
    event_url: String,
    image_url: String,
    country: String,
    city: String,
    description: String,
    year: u64,
    // NOTE: These dates are in the format "DD-MonthShortName-YYYY"
    start_date: String,
    end_date: String,
    // TODO: Test for this?
    expiry_date: String,
    supply: u64,
}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf> for PoapOwnershipVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter the event id of the POAP you want to verify ownership of."
                .to_string(),
            signature: "Sign a statement attesting to ownership of the POAP.".to_string(),
            witness: "Send the attestation and the signature to the witness and issue a credential"
                .to_string(),
            statement_schema: schema_for!(Stmt),
            witness_schema: schema_for!(Prf),
        })
    }

    async fn statement<I: Issuer>(
        &self,
        stmt: &Stmt,
        issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        self.sanity_check(&stmt.issued_at)?;

        // TODO: Investigate!
        // Can POAPs be attached to non EIP155 DIDs?
        if let Subjects::Pkh(Pkh::Eip155(_)) = stmt.subject {
        } else {
            return Err(FlowError::Validation(
                "Currently only supports Ethereum Addresses for POAP Ownership flow".to_string(),
            ));
        }

        let s = stmt.generate_statement()?;

        // The witness takes the statement which is bound to a specific time by the "issued_at"
        // timestamp, places the challenge delimiter in the middle, then adds their own version
        // of the challenge. This ensures that the expected address is the one making this
        // request and this request isn't being replayed from an interaction older than the
        // max_elapsed_minutes.
        Ok(StatementResponse {
            statement: format!(
                "{}{}{}",
                s,
                self.challenge_delimiter,
                issuer.sign(&s).await?
            ),
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, issuer: &I) -> Result<Ctnt, FlowError> {
        self.sanity_check(&proof.statement.issued_at)?;

        let u = Url::parse(&format!(
            "https://api.poap.tech/actions/scan/{}",
            proof.statement.subject.display_id()?
        ))
        .map_err(|e| FlowError::BadLookup(format!("Failed in API request: {}", e)))?;

        let mut headers = HeaderMap::new();
        let hv: HeaderValue = self
            .api_key
            .parse()
            .map_err(|_e| FlowError::BadLookup("Could not parse Header value".to_string()))?;
        let hn: HeaderName = "X-API-KEY"
            .to_string()
            .parse()
            .map_err(|_e| FlowError::BadLookup("Could not parse Header name".to_string()))?;
        headers.insert(hn, hv);

        let client = Client::new();
        let res: Vec<PoapResEntry> = client
            .get(u)
            .headers(headers)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        let mut found = false;
        for entry in res {
            if entry.event.id == proof.statement.event_id {
                found = true;
                break;
            }
        }

        if !found {
            return Err(FlowError::BadLookup(format!(
                "Found no event with id {} in user's POAPs.",
                proof.statement.event_id
            )));
        }

        let s = proof.statement.generate_statement()?;

        proof
            .statement
            .subject
            .valid_signature(
                // Because the timestamp is within the expected bounds, the witness
                // then can recreate the statement by recreating the challenge.
                // This is not vulnerable to replay attacks after the
                // max_elapsed_minutes has elapsed.
                &format!(
                    "{}{}{}",
                    s,
                    &self.challenge_delimiter,
                    issuer.sign(&s).await?
                ),
                &proof.signature,
            )
            .await?;

        Ok(proof.to_content(&s, &proof.signature)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_util::util::{
            test_eth_did, test_witness_signature, test_witness_statement, MockFlow, MockIssuer,
            TestKey, TestWitness,
        },
        types::{
            defs::{Issuer, Proof, Statement, Subject},
            enums::subject::Subjects,
        },
    };

    fn mock_proof(key: fn() -> Subjects, signature: String) -> Prf {
        Prf {
            statement: Stmt {
                subject: key(),
                event_id: 102213,
                issued_at: "2023-09-27T16:36:33.696Z".to_string(),
            },
            signature,
        }
    }

    #[async_trait(?Send)]
    impl Flow<Ctnt, Stmt, Prf> for MockFlow {
        fn instructions(&self) -> Result<Instructions, FlowError> {
            Ok(Instructions {
                statement: "Unimplemented".to_string(),
                statement_schema: schema_for!(Stmt),
                signature: "Unimplemented".to_string(),
                witness: "Unimplemented".to_string(),
                witness_schema: schema_for!(Prf),
            })
        }

        async fn statement<I: Issuer>(
            &self,
            statement: &Stmt,
            _issuer: &I,
        ) -> Result<StatementResponse, FlowError> {
            Ok(StatementResponse {
                statement: statement.generate_statement()?,
                delimiter: Some("\n\n".to_string()),
            })
        }

        async fn validate_proof<I: Issuer>(
            &self,
            proof: &Prf,
            _issuer: &I,
        ) -> Result<Ctnt, FlowError> {
            proof
                .statement
                .subject
                .valid_signature(&self.statement, &self.signature)
                .await?;

            Ok(proof
                .to_content(&self.statement, &self.signature)
                .map_err(FlowError::Proof)?)
        }
    }

    #[tokio::test]
    async fn mock_poap_ownership() {
        let signature = test_witness_signature(TestWitness::NftOwnership, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::NftOwnership, TestKey::Eth).unwrap();

        let p = mock_proof(test_eth_did, signature.clone());

        let flow = MockFlow {
            statement,
            signature,
        };

        let i = MockIssuer {};
        flow.unsigned_credential(&p, &test_eth_did(), &i)
            .await
            .unwrap();
    }
}
