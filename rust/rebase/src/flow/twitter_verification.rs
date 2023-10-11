use crate::{
    content::twitter_verification::TwitterVerificationContent as Ctnt,
    proof::twitter_verification::TwitterVerificationProof as Prf,
    statement::twitter_verification::TwitterVerificationStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        error::FlowError,
    },
};

use async_trait::async_trait;

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client,
};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TwitterVerificationFlow {
    pub api_key: String,
    pub delimiter: String,
}

#[derive(Deserialize, Serialize)]
pub struct TwitterResponseData {
    pub text: String,
}

#[derive(Deserialize, Serialize)]
pub struct TwitterResponseUser {
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct TwitterResponseIncludes {
    pub users: Vec<TwitterResponseUser>,
}

#[derive(Deserialize, Serialize)]
pub struct TwitterResponse {
    pub data: Vec<TwitterResponseData>,
    pub includes: TwitterResponseIncludes,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Flow<Ctnt, Stmt, Prf> for TwitterVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter your Twitter account handle to verify and include in a signed message using your wallet.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your Twitter handle and additional information.".to_string(),
            witness: "Tweet out the statement and signature to create a link between your identifier and Twitter handle.".to_string(),
            witness_schema: schema_for!(Prf),
        })
    }

    async fn statement<I: Issuer + Send + Clone>(
        &self,
        statement: Stmt,
        _issuer: I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            delimiter: Some(self.delimiter.to_owned()),
            statement: statement.generate_statement()?,
        })
    }

    async fn validate_proof<I: Issuer + Send>(
        &self,
        proof: Prf,
        _issuer: I,
    ) -> Result<Ctnt, FlowError> {
        let mut headers = HeaderMap::new();
        let s: HeaderValue = format!("Bearer {}", &self.api_key).parse().map_err(|_| {
            FlowError::BadLookup("failed to generate authorization header".to_string())
        })?;
        headers.insert(AUTHORIZATION, s);
        let client = Client::new();

        let url_vec: Vec<&str> = proof.tweet_url.split('/').collect();
        if url_vec.is_empty() {
            return Err(FlowError::Validation("could not find tweet id".to_owned()));
        }

        let tweet_id = url_vec[url_vec.len() - 1];

        let res: TwitterResponse = client
            .get(
                Url::parse("https://api.twitter.com/2/tweets")
                    .map_err(|e| FlowError::BadLookup(e.to_string()))?,
            )
            .query(&[
                ("ids", tweet_id.to_owned()),
                ("expansions", "author_id".to_string()),
                ("user.fields", "username".to_string()),
            ])
            .headers(headers)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        if res.includes.users.is_empty() {
            return Err(FlowError::BadLookup("No users found".to_string()));
        };

        if proof.statement.handle.to_lowercase() != res.includes.users[0].username.to_lowercase() {
            return Err(FlowError::BadLookup(format!(
                "unexpected handle, wanted: {} got: {}",
                proof.statement.handle.to_lowercase(),
                res.includes.users[0].username.to_lowercase()
            )));
        };

        if res.data.is_empty() {
            return Err(FlowError::BadLookup("No users found".to_string()));
        };

        let mut a = res.data[0].text.split(&self.delimiter);
        let maybe_stmt = a.next();
        let maybe_sig = a.next();
        match (maybe_stmt, maybe_sig) {
            (Some(stmt), Some(sig)) => {
                proof.statement.subject.valid_signature(stmt, sig).await?;
                Ok(proof.to_content(stmt, sig)?)
            }
            _ => Err(FlowError::Validation(
                "Could not parse signature and statement from tweet".to_owned(),
            )),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_util::util::{
            test_did_keypair, test_ed25519_did, test_eth_did, test_solana_did,
            test_witness_signature, test_witness_statement, MockFlow, MockIssuer, TestKey,
            TestWitness,
        },
        types::{
            defs::{Issuer, Proof, Statement, Subject},
            enums::subject::Subjects,
        },
    };

    fn mock_proof(key: fn() -> Subjects) -> Prf {
        Prf {
            statement: Stmt {
                subject: key(),
                handle: "foo".to_owned(),
            },
            tweet_url: "not_tested".to_owned(),
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
                delimiter: Some("\n\n".to_owned()),
            })
        }

        async fn validate_proof<I: Issuer + Send>(
            &self,
            proof: Prf,
            _issuer: I,
        ) -> Result<Ctnt, FlowError> {
            // NOTE: This just passes through, instead of looking up!!!
            if self.statement != proof.statement.generate_statement()? {
                return Err(FlowError::BadLookup("Mismatched statements".to_string()));
            }

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
    async fn mock_twitter() {
        let did = mock_proof(test_eth_did);
        let signature = test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap();

        let flow = MockFlow {
            statement,
            signature,
        };
        let i = MockIssuer {};
        flow.unsigned_credential(did, test_eth_did(), i.clone())
            .await
            .unwrap();

        let did = mock_proof(test_ed25519_did);

        let signature = test_witness_signature(TestWitness::Twitter, TestKey::Ed25519).unwrap();
        let statement = test_witness_statement(TestWitness::Twitter, TestKey::Ed25519).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(did, test_ed25519_did(), i.clone())
            .await
            .unwrap();

        let did = mock_proof(test_solana_did);
        let signature = test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap();
        let statement = test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(did, test_solana_did(), i.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn mock_twitter_on_the_fly() {
        let i = MockIssuer {};
        let (subj1, iss1) = test_did_keypair().await.unwrap();

        let ver_proof1 = Prf {
            statement: Stmt {
                subject: subj1.clone(),
                handle: "foo".to_owned(),
            },
            tweet_url: "unused".to_owned(),
        };

        let statement = ver_proof1.generate_statement().unwrap();
        let signature = iss1.sign(&statement).await.unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };

        flow.unsigned_credential(ver_proof1, subj1, i.clone())
            .await
            .unwrap();

        let (subj2, iss2) = test_did_keypair().await.unwrap();

        let ver_proof2 = Prf {
            statement: Stmt {
                subject: subj2.clone(),
                handle: "foo".to_owned(),
            },
            tweet_url: "unused".to_owned(),
        };

        let statement = ver_proof2.generate_statement().unwrap();
        let signature = iss2.sign(&statement).await.unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };

        flow.unsigned_credential(ver_proof2.clone(), subj2.clone(), i.clone())
            .await
            .unwrap();

        // Make sure it fails correctly:
        let statement = ver_proof2.generate_statement().unwrap();
        let signature = iss1.sign(&statement).await.unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };

        if flow.unsigned_credential(ver_proof2, subj2, i).await.is_ok() {
            panic!("Approved bad signature");
        };
    }
}
