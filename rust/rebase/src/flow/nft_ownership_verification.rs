use crate::{
    content::nft_ownership_verification::NftOwnershipVerificationContent as Ctnt,
    proof::nft_ownership_verification::NftOwnershipVerificationProof as Prf,
    statement::nft_ownership_verification::NftOwnershipVerificationStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        enums::subject::{Pkh, Subjects},
        error::FlowError,
    },
};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use reqwest::Client;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(untagged)]
// NOTE: If adding other providers change `untagged` to something else.
// NOTE: The above change would be a breaking change.
pub enum NftOwnershipVerificationFlow {
    Alchemy(Alchemy),
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Flow<Ctnt, Stmt, Prf> for NftOwnershipVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        match self {
            NftOwnershipVerificationFlow::Alchemy(x) => x.instructions(),
        }
    }

    async fn statement<I: Issuer + Send + Clone>(
        &self,
        stmt: Stmt,
        issuer: I,
    ) -> Result<StatementResponse, FlowError> {
        match self {
            NftOwnershipVerificationFlow::Alchemy(x) => x.statement(stmt, issuer).await,
        }
    }

    async fn validate_proof<I: Issuer + Send>(
        &self,
        proof: Prf,
        issuer: I,
    ) -> Result<Ctnt, FlowError> {
        match self {
            NftOwnershipVerificationFlow::Alchemy(x) => x.validate_proof(proof, issuer).await,
        }
    }
}

// TODO: Make Alchemy variant be configurable by chain + per-chain configs.
// NOTE: For now, this is just a wrapper around the alchemy API.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
pub struct Alchemy {
    pub api_key: String,
    pub challenge_delimiter: String,
    // The amount of time that can pass before the witness
    // wants a new flow initiated. In demo, set to 15 mins.
    // This is checked for a negative value or 0 and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    pub max_elapsed_minutes: i64,
}

pub struct AlchemyPageResult {
    next_page: Option<String>,
    found: bool,
    // If more data is needed about the NFT pass this structure around.
    // res: AlchemyNftRes,
}

impl Alchemy {
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

    pub async fn process_page(
        &self,
        client: &reqwest::Client,
        contract_address: &str,
        base_url: &str,
        page_key: Option<String>,
    ) -> Result<AlchemyPageResult, FlowError> {
        let u = match page_key {
            None => Url::parse(base_url).map_err(|e| FlowError::BadLookup(e.to_string()))?,
            Some(pk) => Url::parse(&format!("{}&pageKey={}", base_url, pk)).map_err(|_e| {
                FlowError::BadLookup("Could not follow paginated results".to_string())
            })?,
        };

        let res: AlchemyNftRes = client
            .get(u)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        let mut result = AlchemyPageResult {
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

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Flow<Ctnt, Stmt, Prf> for Alchemy {
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

    async fn statement<I: Issuer + Send + Clone>(
        &self,
        stmt: Stmt,
        issuer: I,
    ) -> Result<StatementResponse, FlowError> {
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
        let f = issuer.sign(&s);
        let sig = f.await?;
        Ok(StatementResponse {
            statement: format!("{}{}{}", s, self.challenge_delimiter, sig),
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer + Send>(
        &self,
        proof: Prf,
        issuer: I,
    ) -> Result<Ctnt, FlowError> {
        self.sanity_check(&proof.statement.issued_at)?;

        let base = format!(
            "https://{}.g.alchemy.com/nft/v2/{}/getNFTs?owner={}&withMetadata=false",
            proof.statement.network.to_string(),
            self.api_key,
            proof.statement.subject.display_id()?
        );

        let client = Client::new();

        let f = self.process_page(&client, &proof.statement.contract_address, &base, None);
        let mut res = f.await?;

        if !res.found && res.next_page.is_some() {
            loop {
                let f = self.process_page(
                    &client,
                    &proof.statement.contract_address,
                    &base,
                    res.next_page,
                );
                res = f.await?;

                if res.found || res.next_page.is_none() {
                    break;
                }
            }
        }

        if !res.found {
            return Err(FlowError::BadLookup(format!(
                "Found no owned NFTs from contract {}",
                proof.statement.contract_address
            )));
        }

        let s = proof.statement.generate_statement()?;

        let f = issuer.sign(&s);
        let sig = f.await?;
        proof
            .statement
            .subject
            .valid_signature(
                // Because the timestamp is within the expected bounds, the witness
                // then can recreate the statement by recreating the challenge.
                // This is not vulnerable to replay attacks after the
                // max_elapsed_minutes has elapsed.
                &format!("{}{}{}", s, &self.challenge_delimiter, sig),
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
                contract_address: "0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85".to_owned(),
                network: crate::types::defs::AlchemyNetworks::EthMainnet,
                issued_at: "2023-09-27T16:23:00.447Z".to_string(),
            },
            signature,
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
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

        async fn statement<I: Issuer + Send + Clone>(
            &self,
            statement: Stmt,
            _issuer: I,
        ) -> Result<StatementResponse, FlowError> {
            Ok(StatementResponse {
                statement: statement.generate_statement()?,
                delimiter: Some("\n\n".to_string()),
            })
        }

        async fn validate_proof<I: Issuer + Send>(
            &self,
            proof: Prf,
            _issuer: I,
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
    async fn mock_nft_ownership() {
        let signature = test_witness_signature(TestWitness::NftOwnership, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::NftOwnership, TestKey::Eth).unwrap();

        let p = mock_proof(test_eth_did, signature.clone());

        let flow = MockFlow {
            statement,
            signature,
        };

        let i = MockIssuer {};
        flow.unsigned_credential(p, test_eth_did(), i)
            .await
            .unwrap();
    }
}
