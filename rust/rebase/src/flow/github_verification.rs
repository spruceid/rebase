use crate::{
    content::github_verification::GitHubVerificationContent as Ctnt,
    proof::github_verification::GitHubVerificationProof as Prf,
    statement::github_verification::GitHubVerificationStatement as Stmt,
    types::{
        error::FlowError,
        defs::{Flow, StatementResponse, Issuer, Proof, Statement, Subject, Instructions},
    },
};

use async_trait::async_trait;
use regex::Regex;
use reqwest::{
    Client,
    header::{HeaderMap, USER_AGENT}
};
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use serde_json::map::Map;
use ts_rs::TS;
use url::Url;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct GitHubVerificationFlow {
    pub user_agent: String,
    pub delimiter: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GitHubResponse {
    // This value here is { content: String }
    pub files: Map<String, serde_json::value::Value>,
    // TODO: Use serde_with and get better typing?
    // pub files: Map<String, GistContent>,
    pub owner: Owner,
    pub history: Vec<History>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Owner {
    pub login: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct History {
    pub version: String,
}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf> for GitHubVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions { 
            statement: "Enter your GitHub account handle to verify and include in a signed message using your wallet.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your GitHub handle and addtional information.".to_string(),
            witness: "Create a Gist with this message to create a link between your identifier and your GitHub handle.".to_string(),
            witness_schema: schema_for!(Prf) 
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &Stmt,
        _issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            statement: statement.generate_statement()?,
            delimiter: Some(self.delimiter.to_owned())
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, _issuer: &I) -> Result<Ctnt, FlowError> {
        let client = Client::new();
        let request_url = format!("https://api.github.com/gists/{}", proof.gist_id);
        let re = Regex::new(r"^[a-zA-Z0-9]{32}$")
            .map_err(|_| FlowError::BadLookup("could not generate gist id regex".to_string()))?;

        if !re.is_match(&proof.gist_id) {
            return Err(FlowError::BadLookup("gist id invalid".to_string()));
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            self.user_agent.to_string().parse().map_err(|_| {
                FlowError::BadLookup("could not generate header for lookup".to_string())
            })?,
        );

        let res: GitHubResponse = client
            .get(Url::parse(&request_url).map_err(|e| FlowError::BadLookup(e.to_string()))?)
            .headers(headers)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        if proof.statement.handle.to_lowercase() != res.owner.login.to_lowercase() {
            return Err(FlowError::BadLookup(format!(
                "handle mismatch, expected: {}, got: {}",
                proof.statement.handle.to_lowercase(),
                res.owner.login.to_lowercase()
            )));
        };
        let s = serde_json::to_string(&res.files)
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        for (_k, v) in res.files {
            let object = match v.as_object() {
                None => continue,
                Some(x) => x,
            };

            let str_val = match object.get("content") {
                None => continue,
                Some(x) => x,
            };

            let p = match str_val.as_str() {
                None => continue,
                Some(x) => x,
            };

            let mut a = p.split(&self.delimiter); 
            let txt = a.next(); 
            let txt_sig = a.next();

            match (txt, txt_sig) {
                (Some(stmt), Some(sig)) => {
                    if stmt != proof.statement.generate_statement()? {
                        continue;
                    }
                    proof.statement.subject.valid_signature(stmt, sig).await?;
                    return Ok(proof.to_content(stmt, sig)?)
                }
                _ => continue
            }
            
        }

        Err(FlowError::BadLookup(
            // "Failed to find properly formatted gist".to_string(),
            format!("Failed to find files in: {}", s),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_util::util::{
            test_ed25519_did, 
            test_eth_did, test_solana_did, test_witness_signature, MockFlow,
            MockIssuer, TestKey, TestWitness, test_witness_statement,
        },
        types::{
            enums::subject::Subjects,
            defs::{Issuer, Proof, Statement, Subject},
        },
    };

      fn mock_proof(key: fn() -> Subjects) -> Prf {
        Prf {
            statement: Stmt {
                subject: key(),
                handle: "foo".to_owned(),
            },
            gist_id: "not_tested".to_owned(),
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
                witness_schema: schema_for!(Prf)
            })
        }

        async fn statement<I: Issuer>(
            &self,
            statement: &Stmt,
            _issuer: &I,
        ) -> Result<StatementResponse, FlowError> {
            Ok(StatementResponse {
                statement: statement.generate_statement()?,
                delimiter: Some("\n\n".to_string())
            })
        }

        async fn validate_proof<I: Issuer>(
            &self,
            proof: &Prf,
            _issuer: &I,
        ) -> Result<Ctnt, FlowError> {
            let comp = proof.statement.generate_statement()?;
            // NOTE: This just passes through, instead of looking up!!!
            if self.statement != comp {
                return Err(FlowError::BadLookup(format!("Mismatched statements, self: {}, proof: {}", self.statement, comp)))
            }

            proof.statement.subject.valid_signature(&self.statement, &self.signature).await?;

            Ok(proof
                .to_content(&self.statement, &self.signature)
                .map_err(FlowError::Proof)?)
        }
    }

    #[tokio::test]
    async fn mock_github() {
        let did = mock_proof(test_eth_did);
        let signature = test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap();

        let flow = MockFlow {
            statement,
            signature,
        };
        let i = MockIssuer {};
        flow.unsigned_credential(&did, &test_eth_did(), &i)
            .await
            .unwrap();

        let did = mock_proof(test_ed25519_did);

        let signature = test_witness_signature(TestWitness::GitHub, TestKey::Ed25519).unwrap();
        let statement = test_witness_statement(TestWitness::GitHub, TestKey::Ed25519).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };   
        flow.unsigned_credential(&did, &test_ed25519_did(), &i)
            .await
            .unwrap();

        let did = mock_proof(test_solana_did);
        let signature = test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap();
        let statement = test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(&did, &test_solana_did(), &i)
            .await
            .unwrap();
    }
}