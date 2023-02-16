use crate::{
    content::poap_ownership::PoapOwnership as Ctnt,
    proof::poap_ownership::PoapOwnership as Prf,
    statement::poap_ownership::PoapOwnership as Stmt,
    types::{
        enums::subject::{Pkh, Subjects},
        error::FlowError,
        types::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
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
    // The amount of time that can pass before the witness
    // wants a new flow initiated. In demo, set to 15 mins.
    // This is checked for a negative value or 0 and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    max_elapsed_minutes: i64,
}

impl PoapOwnership {
    // NOTE: This method would be vulnerable to someone foward-dating signatures.
    // It likely wouldn't occur, but could be mitigated by doing a Challenge { challenge: string, timestamp: string}
    // and attaching it to FlowResponse with a challenge: Option<Challenge> field.
    // Then, we generate the TS here, like in email, but send it back over the wire as part of the statement.
    // That said, there's no motivation for commiting that style of attack in the current NFT gating demo situation.
    // NOTE: People with clocks that are off might mess this up too.
    // TODO: When moving to post-demo impl, rework this to use the above strategy
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
            return Err(FlowError::Validation(format!(
                "Timestamp provided comes from the future"
            )));
        }

        if now - Duration::minutes(self.max_elapsed_minutes) > then {
            return Err(FlowError::Validation(format!(
                "Validation window has expired"
            )));
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
        _issuer: &I,
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

        Ok(FlowResponse {
            statement: stmt.generate_statement()?,
            delimitor: None,
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, _issuer: &I) -> Result<Ctnt, FlowError> {
        self.sanity_check(&proof.statement.issued_at)?;

        let u = Url::parse(&format!(
            "https://api.poap.tech/actions/scan/{}",
            proof.statement.subject.display_id()?
        ))
        .map_err(|e| FlowError::BadLookup(format!("Failed in API request: {}", e.to_string())))?;

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

        // NOTE: We would generate and append the challenge
        // here if using that scheme.
        proof
            .statement
            .subject
            .valid_signature(&s, &proof.signature)
            .await?;

        Ok(proof.to_content(&s, &proof.signature)?)
    }
}
