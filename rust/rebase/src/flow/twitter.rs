use crate::{
    content::twitter::Twitter as Ctnt,
    proof::twitter::Twitter as Prf,
    statement::twitter::Twitter as Stmt,
    types::{
        defs::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
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
use url::Url;

#[derive(Clone, Deserialize, Serialize)]
pub struct TwitterFlow {
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

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf> for TwitterFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter your Twitter account handle to verify and include in a signed message using your wallet.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your Twitter handle and additional information.".to_string(),
            witness: "Tweet out the statement and signature to create a link between your identifier and Twitter handle.".to_string(),
            witness_schema: schema_for!(Prf),
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &Stmt,
        _issuer: &I,
    ) -> Result<FlowResponse, FlowError> {
        Ok(FlowResponse {
            delimiter: Some(self.delimiter.to_owned()),
            statement: statement.generate_statement()?,
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, _issuer: &I) -> Result<Ctnt, FlowError> {
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
            test_ed25519_did, test_eth_did, test_solana_did, test_witness_signature,
            test_witness_statement, MockFlow, MockIssuer, TestKey, TestWitness,
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
        ) -> Result<FlowResponse, FlowError> {
            Ok(FlowResponse {
                statement: statement.generate_statement()?,
                delimiter: Some("\n\n".to_owned()),
            })
        }

        async fn validate_proof<I: Issuer>(
            &self,
            proof: &Prf,
            _issuer: &I,
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
        flow.unsigned_credential(&did, &test_eth_did(), &i)
            .await
            .unwrap();

        let did = mock_proof(test_ed25519_did);

        let signature = test_witness_signature(TestWitness::Twitter, TestKey::Ed25519).unwrap();
        let statement = test_witness_statement(TestWitness::Twitter, TestKey::Ed25519).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(&did, &test_ed25519_did(), &i)
            .await
            .unwrap();

        let did = mock_proof(test_solana_did);
        let signature = test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap();
        let statement = test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(&did, &test_solana_did(), &i)
            .await
            .unwrap();
    }
}
