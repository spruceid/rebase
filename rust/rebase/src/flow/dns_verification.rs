use crate::{
    content::dns_verification::DnsVerificationContent as Ctnt,
    statement::dns_verification::DnsVerificationStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        error::FlowError,
    },
};

use async_trait::async_trait;
use reqwest::Client;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DnsVerificationFlow {}

#[derive(Deserialize, Debug)]
pub struct DnsResponse {
    #[serde(rename = "Answer")]
    pub answer: Vec<AnswerResponse>,
}

#[derive(Deserialize, Debug)]
pub struct AnswerResponse {
    pub name: String,
    pub data: String,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Flow<Ctnt, Stmt, Stmt> for DnsVerificationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Enter the Web Domain you wish to prove ownership of.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your domain and additional information.".to_string(),
            witness: "In your DNS settings, add a new TXT record for @ and copy and put the following message as the value. Keep in mind that DNS propagation can take some time. This process may take a few minutes for the verification to successfully complete.".to_string(),
            witness_schema: schema_for!(Stmt)
        })
    }

    async fn statement<I: Issuer + Send>(
        &self,
        statement: Stmt,
        _issuer: I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer + Send>(
        &self,
        proof: Stmt,
        _issuer: I,
    ) -> Result<Ctnt, FlowError> {
        let client = Client::new();
        let request_url = format!(
            "https://cloudflare-dns.com/dns-query?name={}&type=txt",
            &proof.domain
        );

        let res: DnsResponse = client
            .get(Url::parse(&request_url).map_err(|e| FlowError::BadLookup(e.to_string()))?)
            .header("accept", "application/dns-json")
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        for answer in res.answer {
            let mut sig: &str = &answer.data;
            if sig.starts_with('"') && sig.ends_with('"') {
                sig = &answer.data[1..answer.data.len() - 1];
            }

            if sig.starts_with(&proof.prefix) {
                sig = sig.trim_start_matches(&proof.prefix);
                let stmt = proof.generate_statement()?;
                proof.subject.valid_signature(&stmt, sig).await?;
                return Ok(proof.to_content(&stmt, sig)?);
            }
        }

        Err(FlowError::BadLookup(
            "expected record not found".to_string(),
        ))
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

    fn mock_proof(key: fn() -> Subjects) -> Stmt {
        Stmt {
            subject: key(),
            domain: "example.com".to_owned(),
            prefix: "not_needed".to_owned(),
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
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

        async fn statement<I: Issuer + Send>(
            &self,
            statement: Stmt,
            _issuer: I,
        ) -> Result<StatementResponse, FlowError> {
            Ok(StatementResponse {
                statement: statement.generate_statement()?,
                delimiter: None,
            })
        }

        async fn validate_proof<I: Issuer + Send>(
            &self,
            proof: Stmt,
            _issuer: I,
        ) -> Result<Ctnt, FlowError> {
            // NOTE: This just passes through, instead of looking up!!!
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
    async fn mock_dns() {
        let did = mock_proof(test_eth_did);
        let signature = test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap();
        let statement = test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap();

        let flow = MockFlow {
            statement,
            signature,
        };
        let i = MockIssuer {};
        flow.unsigned_credential(did.clone(), test_eth_did(), i.clone())
            .await
            .unwrap();

        let did = mock_proof(test_ed25519_did);
        let signature = test_witness_signature(TestWitness::DNS, TestKey::Ed25519).unwrap();
        let statement = test_witness_statement(TestWitness::DNS, TestKey::Ed25519).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(did.clone(), test_ed25519_did(), i.clone())
            .await
            .unwrap();

        let did = mock_proof(test_solana_did);
        let signature = test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap();
        let statement = test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };
        flow.unsigned_credential(did.clone(), test_solana_did(), i.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn mock_dns_on_the_fly() {
        let i = MockIssuer {};
        let (subj1, iss1) = test_did_keypair().await.unwrap();

        let ver_stmt1 = Stmt {
            subject: subj1.clone(),
            domain: "example.com".to_owned(),
            prefix: "not_needed".to_owned(),
        };

        let statement = ver_stmt1.generate_statement().unwrap();
        let signature = iss1.sign(&statement).await.unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };

        flow.unsigned_credential(ver_stmt1.clone(), subj1.clone(), i.clone())
            .await
            .unwrap();

        let (subj2, iss2) = test_did_keypair().await.unwrap();

        let ver_stmt2 = Stmt {
            subject: subj2.clone(),
            domain: "example.com".to_owned(),
            prefix: "not_needed".to_owned(),
        };

        let statement = ver_stmt2.generate_statement().unwrap();
        let signature = iss2.sign(&statement).await.unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };

        flow.unsigned_credential(ver_stmt2.clone(), subj2.clone(), i.clone())
            .await
            .unwrap();

        // Make sure it fails correctly:
        let statement = ver_stmt2.generate_statement().unwrap();
        let signature = iss1.sign(&statement).await.unwrap();
        let flow = MockFlow {
            statement,
            signature,
        };

        if flow.unsigned_credential(ver_stmt2, subj2, i).await.is_ok() {
            panic!("Approved bad signature");
        };
    }
}
