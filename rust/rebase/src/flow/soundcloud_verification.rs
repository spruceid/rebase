use crate::{
    content::soundcloud_verification::SoundCloudVerificationContent as Ctnt,
    statement::soundcloud_verification::SoundCloudVerificationStatement as Stmt,
    types::{
        defs::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
        error::FlowError,
    },
};

use async_trait::async_trait;
use reqwest::Client;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use url::Url;

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SoundCloudVerificationFlow {
    pub client_id: String,
    // Must be less than 200
    pub limit: u64,
    // Must be less that 10000 If less than limit, will only make one request.
    pub max_offset: u64,
}

impl SoundCloudVerificationFlow {
    fn is_valid(&self) -> Result<(), FlowError> {
        if self.limit > 200 {
            Err(FlowError::Validation(
                "limit must be less than or equal to 200".to_string(),
            ))
        } else if self.limit == 0 {
            Err(FlowError::Validation(
                "limit must be greater than 0".to_string(),
            ))
        } else if (self.max_offset + self.limit) > 10000 {
            Err(FlowError::Validation(
                "the sum of max_offset and limit must be less than 10000".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn generate_url(&self, proof: &Stmt, offset: &u64) -> Result<Url, FlowError> {
        Url::parse(&format!(
            "https://api-v2.soundcloud.com/search/users?q={}&client_id={}&limit={}&offset={}&app_locale=en",
            proof.permalink,
            self.client_id,
            self.limit,
            offset
        )).map_err(|e| FlowError::BadLookup(format!("could not parse generated url, reason: {}", e)))
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct SoundCloudRes {
    pub collection: Vec<SoundCloudEntry>,
}

#[derive(Deserialize, Debug, Serialize)]
struct SoundCloudEntry {
    pub permalink: Option<String>,
    pub description: Option<String>,
}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Stmt> for SoundCloudVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter your SoundCloud profile url to verify and include in a signed message using your wallet.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your SoundCloud handle and additional information.".to_string(),
            witness: "Update your SoundCloud profile's Bio section to include only the signature shown.".to_string(),
            witness_schema: schema_for!(Stmt),
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &Stmt,
        _issuer: &I,
    ) -> Result<FlowResponse, FlowError> {
        Ok(FlowResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &Stmt,
        _issuer: &I,
    ) -> Result<Ctnt, FlowError> {
        self.is_valid()?;
        let mut offset = 0;
        let client = Client::new();

        while offset <= self.max_offset {
            let u = self.generate_url(proof, &offset)?;
            let res: SoundCloudRes = client
                .get(u)
                .send()
                .await
                .map_err(|e| FlowError::BadLookup(e.to_string()))?
                .json()
                .await
                .map_err(|e| FlowError::BadLookup(e.to_string()))?;

            if res.collection.is_empty() {
                break;
            }

            for entry in res.collection {
                if let Some(permalink) = entry.permalink {
                    if permalink.to_lowercase() == proof.permalink.to_lowercase() {
                        if let Some(description) = entry.description {
                            let stmt = proof.generate_statement()?;
                            proof.subject.valid_signature(&stmt, &description).await?;
                            return Ok(proof.to_content(&stmt, &description)?);
                        }
                    }
                }
            }

            offset += self.limit;
        }

        Err(FlowError::BadLookup(format!(
            "soundcloud profile {} not found after searching up to {} entries",
            proof.permalink,
            self.max_offset + self.limit
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_util::util::{
            // test_ed25519_did,
            test_eth_did,
            test_solana_did,
            test_witness_signature,
            test_witness_statement,
            MockFlow,
            MockIssuer,
            TestKey,
            TestWitness,
        },
        types::{
            defs::{Issuer, Statement, Subject},
            enums::subject::Subjects,
        },
    };

    fn mock_proof(key: fn() -> Subjects) -> Stmt {
        Stmt {
            subject: key(),
            permalink: "foo".to_owned(),
        }
    }

    #[async_trait(?Send)]
    impl Flow<Ctnt, Stmt, Stmt> for MockFlow {
        fn instructions(&self) -> Result<Instructions, FlowError> {
            Ok(Instructions {
                statement: "Unimplemented".to_string(),
                statement_schema: schema_for!(Stmt),
                signature: "Unimplemented".to_string(),
                witness: "Unimplemented".to_string(),
                witness_schema: schema_for!(Stmt),
            })
        }

        async fn statement<I: Issuer>(
            &self,
            statement: &Stmt,
            _issuer: &I,
        ) -> Result<FlowResponse, FlowError> {
            Ok(FlowResponse {
                statement: statement.generate_statement()?,
                delimiter: None,
            })
        }

        async fn validate_proof<I: Issuer>(
            &self,
            proof: &Stmt,
            _issuer: &I,
        ) -> Result<Ctnt, FlowError> {
            // NOTE: This just passes through, instead of looking up!!!
            if self.statement != proof.generate_statement()? {
                return Err(FlowError::BadLookup("Mismatched statements".to_string()));
            }

            proof
                .subject
                .valid_signature(&self.statement, &self.signature)
                .await?;

            Ok(proof
                .to_content(&self.statement, &self.signature)
                .map_err(FlowError::Proof)?)
        }
    }

    #[tokio::test]
    async fn mock_soundcloud() {
        let did = mock_proof(test_eth_did);
        let signature = test_witness_signature(TestWitness::SoundCloud, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::SoundCloud, TestKey::Eth).unwrap();

        let flow = MockFlow {
            statement,
            signature,
        };
        let i = MockIssuer {};
        flow.unsigned_credential(&did, &test_eth_did(), &i)
            .await
            .unwrap();

        let did = mock_proof(test_solana_did);
        let signature = test_witness_signature(TestWitness::SoundCloud, TestKey::Solana).unwrap();
        let statement = test_witness_statement(TestWitness::SoundCloud, TestKey::Solana).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(&did, &test_solana_did(), &i)
            .await
            .unwrap();
    }
}
