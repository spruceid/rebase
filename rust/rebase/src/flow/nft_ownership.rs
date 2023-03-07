use crate::{
    content::nft_ownership::NftOwnership as Ctnt,
    proof::nft_ownership::NftOwnership as Prf,
    statement::nft_ownership::NftOwnership as Stmt,
    types::{
        defs::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
        enums::subject::{Pkh, Subjects},
        error::FlowError,
    },
};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use url::Url;

// TODO: Make this an Enum of which Alchemy is the only impl.
// TODO: Make Alchemy variant be configurable by chain + per-chain configs.
// NOTE: For now, this is just a wrapper around the alchemy API.
#[derive(Clone, Deserialize, Serialize)]
pub struct NftOwnership {
    api_key: String,
    challenge_delimiter: String,
    // The amount of time that can pass before the witness
    // wants a new flow initiated. In demo, set to 15 mins.
    // This is checked for a negative value or 0 and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    max_elapsed_minutes: i64,
}

pub struct PageResult {
    next_page: Option<String>,
    found: bool,
    // If more data is needed about the NFT pass this structure around.
    // res: AlchemyNftRes,
}

impl NftOwnership {
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

    // TODO: Change so URL gets generated in here.
    pub async fn process_page(
        &self,
        client: &reqwest::Client,
        u: url::Url,
        contract_address: &str,
    ) -> Result<PageResult, FlowError> {
        let res: AlchemyNftRes = client
            .get(u)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        let mut result: PageResult = PageResult {
            next_page: res.page_key.clone(),
            found: false,
            // res: res.clone(),
        };

        for nft in res.owned_nfts {
            if nft.contract.address == contract_address {
                result.found = true;
                break;
            }
        }

        Ok(result)
    }
}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf> for NftOwnership {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter the contract address and network of asset".to_string(),
            signature: "Sign a statement attesting to ownership of the asset".to_string(),
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

        // TODO: Adjust this when adding additional Alchemy flows.
        match stmt.subject {
            Subjects::Pkh(Pkh::Eip155(_)) => {}
            _ => {
                return Err(FlowError::Validation(
                    "Currently only supports Ethereum NFTs".to_string(),
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

    // TODO: Change this whole flow so URL gets generated in process_page.
    async fn validate_proof<I: Issuer>(&self, proof: &Prf, issuer: &I) -> Result<Ctnt, FlowError> {
        self.sanity_check(&proof.statement.issued_at)?;

        let base = format!(
            "https://{}-{}.g.alchemy.com/nft/v2/{}/getNFTs?owner={}&withMetadata=false",
            // TODO: Replace with enum.
            "eth",
            // TODO: Replace with enum.
            proof.statement.network,
            self.api_key,
            proof.statement.subject.display_id()?
        );

        let client = Client::new();

        let mut next_u: url::Url =
            Url::parse(&base).map_err(|e| FlowError::BadLookup(e.to_string()))?;

        let res = loop {
            let inner_res = self
                .process_page(&client, next_u.clone(), &proof.statement.contract_address)
                .await?;

            if inner_res.found {
                break inner_res;
            }

            let page_key = inner_res.next_page.clone();
            match page_key {
                None => break inner_res,
                Some(s) => {
                    next_u = Url::parse(&format!("{}&pageKey={}", base, s)).map_err(|_e| {
                        FlowError::BadLookup("Could not follow paginated results".to_string())
                    })?;
                }
            }
        };

        if !res.found {
            return Err(FlowError::BadLookup(format!(
                "Found no owned NFTs from contract {}",
                proof.statement.contract_address
            )));
        }

        let s = proof.statement.generate_statement()?;
        // NOTE: We would generate and append the challenge
        // here if using that scheme.
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AlchemyNftRes {
    owned_nfts: Vec<AlchemyNftEntry>,
    page_key: Option<String>,
    total_count: i64,
    block_hash: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct AlchemyNftEntry {
    contract: AlchemyNftContractEntry,
    id: AlchemyTokenId,
    // NOTE: Balance always seems to be a number
    balance: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct AlchemyNftContractEntry {
    address: String,
}

#[derive(Clone, Deserialize, Serialize)]
struct AlchemyTokenId {
    #[serde(rename = "tokenId")]
    token_id: String,
}
