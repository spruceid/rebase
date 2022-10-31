use crate::{
    schema::schema_type::{SchemaError, SchemaType},
    signer::signer::{SignerError, SignerType, DID as SignerDID},
    witness::{
        signer_type::SignerTypes,
        witness::{Generator, Proof, Statement, WitnessError},
    },
};
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use std::collections::HashMap;
use url::Url;

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "claim")]
pub struct Claim {
    pub domain: String,
    pub prefix: String,
    pub key_type: SignerDID,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "{} is linked to {}",
            self.domain,
            signer_type.statement_id()?
        ))
    }

    fn delimitor(&self) -> String {
        "=".to_string()
    }
}

impl Proof for Claim {}

pub struct Schema {
    pub domain: String,
    pub key_type: SignerDID,
}

impl SchemaType for Schema {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: Change where these are pointed
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "DnsVerification": "https://example.com/DnsVerification",
                "DnsVerificationMessage": {
                    "@id": "https://example.com/DnsVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "dnsServer": "https://example.com/dnsServer",
                    }
                }
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        let mut evidence_map = HashMap::new();

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        evidence_map.insert(
            "dnsServer".to_string(),
            serde_json::Value::String("https://cloudflare-dns.com/dns-query".to_string()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["DnsVerificationMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        let signer_type = SignerTypes::new(&self.key_type)?;
        let signer_did = signer_type
            .did_id()
            .map_err(|e| SchemaError::BadSubject(e.to_string()))?;

        Ok(json!({
            "id": signer_did.to_owned(),
            "sameAs": format!("dns:{}",  self.domain)
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "DnsVerification".to_string(),
        ])
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ClaimGenerator {}

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

#[async_trait(?Send)]
impl Generator<Claim, Schema> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        let client = reqwest::Client::new();
        let request_url = format!(
            "https://cloudflare-dns.com/dns-query?name={}&type=txt",
            &proof.domain
        );

        let res: DnsResponse = client
            .get(Url::parse(&request_url).map_err(|e| WitnessError::BadLookup(e.to_string()))?)
            .header("accept", "application/dns-json")
            .send()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?;

        for answer in res.answer {
            let mut sig: &str = &answer.data;
            if sig.starts_with('"') && sig.ends_with('"') {
                sig = &answer.data[1..answer.data.len() - 1];
            }

            if sig.starts_with(&proof.prefix) {
                sig = sig.trim_start_matches(&proof.prefix);
                let s = format!(
                    "{}{}{}",
                    proof.generate_statement()?,
                    proof.delimitor(),
                    sig
                );
                return Ok(s);
            }
        }

        Err(WitnessError::BadLookup(
            "expected record not found".to_string(),
        ))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        _statement: &str,
        _signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            domain: proof.domain.clone(),
            key_type: proof.key_type.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signer::ed25519::Ed25519;
    use crate::util::util::{
        test_ed25519_did, test_eth_did, test_solana_did, test_witness_signature, MockGenerator, TestKey, TestWitness,
    };
    use crate::witness::witness::Generator;

    fn mock_proof(key: fn() -> SignerDID) -> Claim {
        Claim {
            key_type: key(),
            domain: "example.com".to_owned(),
            prefix: "not_needed".to_owned(),
        }
    }

    #[async_trait(?Send)]
    impl Generator<Claim, Schema> for MockGenerator {
        async fn locate_post(&self, _proof: &Claim) -> Result<String, WitnessError> {
            Ok(self.post.clone())
        }

        fn _unchecked_to_schema(
            &self,
            proof: &Claim,
            _statement: &str,
            _signature: &str,
        ) -> Result<Schema, WitnessError> {
            Ok(Schema {
                domain: proof.domain.clone(),
                key_type: proof.key_type.clone(),
            })
        }
    }

    #[tokio::test]
    async fn mock_dns() {
        let sig = test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap();
        let did = mock_proof(test_eth_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_eth_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::DNS, TestKey::Ed25519).unwrap();
        let did = mock_proof(test_ed25519_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_ed25519_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap();
        let did = mock_proof(test_solana_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_solana_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();
    }
}
