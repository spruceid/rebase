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
    pub permalink: String,
    pub key_type: SignerDID,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "I am attesting that this SoundCloud profile https://soundcloud.com/{} is linked to the {} {}",
            self.permalink,
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
    pub key_type: SignerDID,
    pub statement: String,
    pub signature: String,
    pub permalink: String,
}

impl SchemaType for Schema {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS POINT ELSEWHERE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "SoundCloudVerification": "https://example.com/SoundCloudVerification",
                "SoundCloudVerificationMessage": {
                    "@id": "https://example.com/SoundCloudVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "permalink": "https://example.com/permalink",
                    }
                }
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "permalink".to_string(),
            serde_json::Value::String(self.permalink.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["SoundCloudVerificationMessage".to_string()],
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
            "sameAs": format!("https://soundcloud.com/{}", self.permalink)
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "SoundCloudVerification".to_owned(),
        ])
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ClaimGenerator {
    pub client_id: String,
    // Must be less than 200
    pub limit: u64,
    // Must be less that 10000 If less than limit, will only make one request.
    pub max_offset: u64,
}

impl ClaimGenerator {
    fn is_valid(&self) -> Result<(), WitnessError> {
        if self.limit > 200 {
            Err(WitnessError::BadConfig(
                "limit must be less than or equal to 200".to_string(),
            ))
        } else if self.limit <= 0 {
            Err(WitnessError::BadConfig(
                "limit must be greater than 0".to_string(),
            ))
        } else if (self.max_offset + self.limit) > 10000 {
            Err(WitnessError::BadConfig(
                "the sum of max_offset and limit must be less than 10000".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn generate_url(&self, proof: &Claim, offset: &u64) -> Result<Url, WitnessError> {
        Url::parse(&format!(
            "https://api-v2.soundcloud.com/search/users?q={}&client_id={}&limit={}&offset={}&app_locale=en",
            proof.permalink,
            self.client_id,
            self.limit,
            offset
        )).map_err(|e| WitnessError::BadLookup(format!("could not parse generated url, reason: {}", e)))
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
impl Generator<Claim, Schema> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        self.is_valid()?;
        let mut offset = 0;
        let client = reqwest::Client::new();

        while offset <= self.max_offset {
            let u = self.generate_url(proof, &offset)?;
            let res: SoundCloudRes = client
                .get(u)
                .send()
                .await
                .map_err(|e| WitnessError::BadLookup(e.to_string()))?
                .json()
                .await
                .map_err(|e| WitnessError::BadLookup(e.to_string()))?;

            if res.collection.len() <= 0 {
                break;
            }

            for entry in res.collection {
                match entry.permalink {
                    Some(permalink) => {
                        if permalink.to_lowercase() == proof.permalink.to_lowercase() {
                            match entry.description {
                                Some(description) => {
                                    return Ok(format!(
                                        "{}{}{}",
                                        proof.generate_statement()?,
                                        proof.delimitor(),
                                        description.clone()
                                    ));
                                }
                                None => {}
                            }
                        }
                    }
                    None => {}
                }
            }

            offset = offset + self.limit;
        }

        Err(WitnessError::BadLookup(format!(
            "soundcloud profile {} not found after searching up to {} entries",
            proof.permalink,
            self.max_offset + self.limit
        )))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        statement: &str,
        signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            // comment_id: proof.comment_id.clone(),
            permalink: proof.permalink.clone(),
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
            permalink: "foo".to_owned(),
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
                permalink: proof.permalink.clone(),
                statement: statement.to_owned(),
                signature: signature.to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn mock_soundcloud() {
        let sig = test_witness_signature(TestWitness::SoundCloud, TestKey::Eth).unwrap();
        let did = mock_proof(test_eth_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_eth_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::SoundCloud, TestKey::Solana).unwrap();
        let did = mock_proof(test_solana_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_solana_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        // NOTE: Does not test Ed25519 Key, like in DNS/Twitter/GitHub/SelfSigned
    }
}
