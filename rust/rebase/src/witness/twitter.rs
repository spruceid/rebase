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
use reqwest::header::{HeaderMap, AUTHORIZATION};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use url::Url;

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "opts")]
pub struct Opts {
    pub handle: String,
    pub key_type: SignerDID,
}

impl Statement for Opts {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "I am attesting that this twitter handle @{} is linked to the {} {}",
            self.handle,
            signer_type.name(),
            signer_type.statement_id()?
        ))
    }

    fn delimitor(&self) -> String {
        "\n\n".to_string()
    }
}

// NOTE: One could impl Proof for Opts but there's no corresponding schema + generator.
// impl Proof for Opts {}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "claim")]
pub struct Claim {
    pub statement_opts: Opts,
    pub tweet_url: String,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        self.statement_opts.signer_type()
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        self.statement_opts.generate_statement()
    }

    fn delimitor(&self) -> String {
        self.statement_opts.delimitor()
    }
}

// TODO: Can we just derive this?
impl Proof for Claim {}

pub struct Schema {
    pub handle: String,
    pub key_type: SignerDID,
    pub tweet_url: String,
    pub statement: String,
    pub signature: String,
}

impl SchemaType for Schema {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: Change where these are pointed
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "TwitterVerification": "https://example.com/TwitterVerification",
                "TwitterVerificationPublicTweet": {
                    "@id": "https://example.com/TwitterVerificationPublicTweet",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "handle": "https://example.com/handle",
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "tweetId": "https://example.com/tweetId"
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
        let url_vec: Vec<&str> = self.tweet_url.split("/").collect();
        if url_vec.len() < 1 {
            return Err(SchemaError::BadSubject(
                "could not find tweet id".to_owned(),
            ));
        }

        let tweet_id = url_vec[url_vec.len() - 1];
        evidence_map.insert(
            "tweetId".to_string(),
            serde_json::Value::String(tweet_id.to_owned()),
        );
        let evidence = Evidence {
            id: None,
            type_: vec!["TwitterVerificationPublicTweet".to_string()],
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
            "sameAs": format!("{}{}", "https://twitter.com/",  self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "TwitterVerification".to_string(),
        ])
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ClaimGenerator {
    pub api_key: String,
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
impl Generator<Claim, Schema> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        let mut headers = HeaderMap::new();
        let s: reqwest::header::HeaderValue =
            format!("Bearer {}", &self.api_key).parse().map_err(|_| {
                WitnessError::BadLookup("failed to generate authorization header".to_string())
            })?;
        headers.insert(AUTHORIZATION, s);
        let client = reqwest::Client::new();

        let url_vec: Vec<&str> = proof.tweet_url.split("/").collect();
        if url_vec.len() < 1 {
            return Err(WitnessError::SchemaError(SchemaError::BadSubject(
                "could not find tweet id".to_owned(),
            )));
        }

        let tweet_id = url_vec[url_vec.len() - 1];

        let res: TwitterResponse = client
            .get(
                Url::parse("https://api.twitter.com/2/tweets")
                    .map_err(|e| WitnessError::BadLookup(e.to_string()))?,
            )
            .query(&[
                ("ids", tweet_id.to_owned()),
                ("expansions", "author_id".to_string()),
                ("user.fields", "username".to_string()),
            ])
            .headers(headers)
            .send()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?;

        if res.includes.users.len() < 1 {
            return Err(WitnessError::BadLookup("No users found".to_string()));
        };

        if proof.statement_opts.handle.to_lowercase()
            != res.includes.users[0].username.to_lowercase()
        {
            return Err(WitnessError::BadLookup(format!(
                "unexpected handle, wanted: {} got: {}",
                proof.statement_opts.handle.to_lowercase(),
                res.includes.users[0].username.to_lowercase()
            )));
        };

        if res.data.len() < 1 {
            return Err(WitnessError::BadLookup("No users found".to_string()));
        };

        Ok(res.data[0].text.to_owned())
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        statement: &str,
        signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            handle: proof.statement_opts.handle.clone(),
            key_type: proof.statement_opts.key_type.clone(),
            tweet_url: proof.tweet_url.clone(),
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
        test_ed25519_did, test_eth_did, test_solana_did, test_witness_signature, MockGenerator, TestKey, TestWitness,
    };
    use crate::witness::witness::Generator;

    fn mock_proof(key: fn() -> SignerDID) -> Claim {
        Claim {
            statement_opts: Opts {
                key_type: key(),
                handle: "foo".to_owned(),
            },
            tweet_url: "not_needed".to_owned(),
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
                tweet_url: proof.tweet_url.clone(),
                key_type: proof.statement_opts.key_type.clone(),
                handle: proof.statement_opts.handle.clone(),
                statement: statement.to_owned(),
                signature: signature.to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn mock_twitter() {
        let sig = test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap();
        let did = mock_proof(test_eth_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_eth_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::Twitter, TestKey::Ed25519).unwrap();
        let did = mock_proof(test_ed25519_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_ed25519_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap();
        let did = mock_proof(test_solana_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_solana_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();
    }
}
