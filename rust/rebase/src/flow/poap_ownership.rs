use crate::{
    content::poap_ownership::PoapOwnership as Ctnt,
    proof::poap_ownership::PoapOwnership as Prf,
    statement::poap_ownership::PoapOwnership as Stmt,
    types::{
        defs::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
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
use url::Url;

#[derive(Clone, Deserialize, Serialize)]
pub struct PoapOwnership {
    api_key: String,
    challenge_delimiter: String,
    // The amount of time that can pass before the witness
    // wants a new flow initiated. In demo, set to 15 mins.
    // This is checked for a negative value or 0 and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    max_elapsed_minutes: i64,
}

impl PoapOwnership {
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
    id: u64,
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
impl Flow<Ctnt, Stmt, Prf> for PoapOwnership {
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
    ) -> Result<FlowResponse, FlowError> {
        self.sanity_check(&stmt.issued_at)?;

        // TODO: Investigate!
        // Can POAPs be attached to non EIP155 DIDs?
        match stmt.subject {
            Subjects::Pkh(Pkh::Eip155(_)) => {}
            _ => {
                return Err(FlowError::Validation(
                    "Currently only supports Ethereum Addresses for POAP Ownership flow"
                        .to_string(),
                ))
            }
        }
        let s = stmt.generate_statement()?;

        // The witness takes the statement which is bound to a specific time by the "issued_at"
        // timestamp, places the challenge delimiter in the middle, then adds their own version
        // of the challenge. This ensures that the expected address is the one making this
        // request and this request isn't being replayed from an interaction older than the
        // max_elapsed_minutes.
        Ok(FlowResponse {
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
