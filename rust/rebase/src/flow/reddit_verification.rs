use crate::{
    content::reddit_verification::RedditVerificationContent as Ctnt,
    statement::reddit_verification::RedditVerificationStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        error::FlowError,
    },
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, Serialize)]
pub struct AboutWrapper {
    pub data: AboutData,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AboutData {
    pub subreddit: AboutSubreddit,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AboutSubreddit {
    pub public_description: String,
}

#[derive(Clone, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RedditVerificationFlow {
    pub user_agent: String,
}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Stmt> for RedditVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter your Reddit account handle to verify and include in a signed message using your wallet.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your Reddit handle and additional information.".to_string(),
            witness: "Update your Reddit profile so that the About section only includes the signature shown.".to_string(),
            witness_schema: schema_for!(Stmt),
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &Stmt,
        _issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &Stmt,
        _issuer: &I,
    ) -> Result<Ctnt, FlowError> {
        let u = format!("https:/www.reddit.com/user/{}/about/.json", proof.handle);
        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            self.user_agent.to_string().parse().map_err(|_| {
                FlowError::BadLookup("could not generate header for lookup".to_string())
            })?,
        );

        let res: AboutWrapper = client
            .get(Url::parse(&u).map_err(|e| {
                FlowError::Validation(format!(
                    "Failed to parse reddit about URL: {} -- Reason: {}",
                    u, e
                ))
            })?)
            .headers(headers)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        let stmt = proof.generate_statement()?;
        let sig = res.data.subreddit.public_description;
        proof.subject.valid_signature(&stmt, &sig).await?;

        Ok(proof.to_content(&stmt, &sig)?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_util::util::{
            test_eth_did, test_solana_did, test_witness_signature, test_witness_statement,
            MockFlow, MockIssuer, TestKey, TestWitness,
        },
        types::{
            defs::{Issuer, Statement, StatementResponse, Subject},
            enums::subject::Subjects,
        },
    };

    fn mock_proof(key: fn() -> Subjects) -> Stmt {
        Stmt {
            subject: key(),
            handle: "foo".to_owned(),
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
        ) -> Result<StatementResponse, FlowError> {
            Ok(StatementResponse {
                statement: statement.generate_statement()?,
                delimiter: None,
            })
        }

        async fn validate_proof<I: Issuer>(
            &self,
            proof: &Stmt,
            _issuer: &I,
        ) -> Result<Ctnt, FlowError> {
            if self.statement != proof.generate_statement()? {
                return Err(FlowError::BadLookup(format!(
                    "Mismatched statements self: {}, proof: {}",
                    self.statement,
                    proof.generate_statement()?
                )));
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
    async fn mock_reddit() {
        let did = mock_proof(test_eth_did);
        let signature = test_witness_signature(TestWitness::Reddit, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::Reddit, TestKey::Eth).unwrap();

        let flow = MockFlow {
            statement,
            signature,
        };
        let i = MockIssuer {};
        flow.unsigned_credential(&did, &test_eth_did(), &i)
            .await
            .unwrap();

        let did = mock_proof(test_solana_did);
        let signature = test_witness_signature(TestWitness::Reddit, TestKey::Solana).unwrap();
        let statement = test_witness_statement(TestWitness::Reddit, TestKey::Solana).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(&did, &test_solana_did(), &i)
            .await
            .unwrap();
    }
}
