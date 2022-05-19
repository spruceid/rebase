use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{SignerError, SignerType, DID as SignerDID};
use crate::witness::{
    signer_type::SignerTypes,
    witness::{Generator, Proof, Statement, WitnessError},
};
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use std::collections::HashMap;
use url::Url;

// TODO: Move to own dir, maybe w/ schema?
// TODO: Add Serde
// TODO: Support the more specific TZProfiles attestation. Requires TZProfiles specific text.

#[derive(Deserialize, Serialize)]
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
            proof.domain
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

        let mut sig = String::new();
        for answer in res.answer {
            let mut trimmed_signature: &str = &answer.data;
            if trimmed_signature.starts_with('"') && trimmed_signature.ends_with('"') {
                trimmed_signature = &answer.data[1..answer.data.len() - 1];
            }
            if trimmed_signature.starts_with(&proof.prefix) {
                sig = trimmed_signature.to_owned();
                break;
            }
        }

        // NOTE: We intercept the post and change it to match the <statement>=<signature>
        // style format.
        Ok(format!("{}={}", proof.generate_statement()?, sig))
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
