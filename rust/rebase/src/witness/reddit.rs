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
use url::Url;

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "claim")]
pub struct Claim {
    pub handle: String,
    pub key_type: SignerDID,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "I am attesting that this Reddit handle {} is linked to the {} {}",
            self.handle,
            signer_type.name(),
            signer_type.statement_id()?
        ))
    }

    fn delimitor(&self) -> String {
        "\n\n".to_string()
    }
}

impl Proof for Claim {}

pub struct Schema {
    pub handle: String,
    pub key_type: SignerDID,
    pub statement: String,
    pub signature: String,
}

impl SchemaType for Schema {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS POINT ELSEWHERE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "RedditVerification": "https://example.com/RedditVerification",
                "RedditVerificationMessage": {
                    "@id": "https://example.com/RedditVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "handle": "https://example.com/handle",
                    }
                }
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "handle".to_string(),
            serde_json::Value::String(self.handle.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["RedditVerificationMessage".to_string()],
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
            "id": signer_did,
            "sameAs": format!("https://reddit.com/user/{}/", self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "RedditVerification".to_owned(),
        ])
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ClaimGenerator {}

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

#[async_trait(?Send)]
impl Generator<Claim, Schema> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        let u = format!("https:/www.reddit.com/user/{}/about/.json", proof.handle);
        let client = reqwest::Client::new();

        let res: AboutWrapper = client
            .get(Url::parse(&u).map_err(|e| {
                WitnessError::ParseError(format!(
                    "Failed to parse reddit about URL: {} -- Reason: {}",
                    u, e
                ))
            })?)
            .send()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?;

        Ok(format!(
            "{}{}{}",
            proof.generate_statement()?,
            proof.delimitor(),
            res.data.subreddit.public_description
        ))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        statement: &str,
        signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            handle: proof.handle.clone(),
            key_type: proof.key_type.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signer::ed25519::Ed25519;
    use crate::util::util::{
        test_ed25519_did, test_eth_did, test_solana_did, test_witness_signature, MockGenerator,
        TestKey, TestWitness,
    };
    use crate::witness::witness::Generator;

    fn mock_proof(key: fn() -> SignerDID) -> Claim {
        Claim {
            key_type: key(),
            handle: "foo".to_owned(),
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
            statement: &str,
            signature: &str,
        ) -> Result<Schema, WitnessError> {
            Ok(Schema {
                key_type: proof.key_type.clone(),
                handle: proof.handle.clone(),
                statement: statement.to_owned(),
                signature: signature.to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn mock_reddit() {
        let sig = test_witness_signature(TestWitness::Reddit, TestKey::Eth).unwrap();
        let did = mock_proof(test_eth_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_eth_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::Reddit, TestKey::Solana).unwrap();
        let did = mock_proof(test_solana_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_solana_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        // NOTE: Does not test Ed25519 Key, like in DNS/Twitter/GitHub/SelfSigned
    }
}
