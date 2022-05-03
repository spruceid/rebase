use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{SignerError, SignerType, DID as SignerDID};
use crate::witness::{
    signer_type::SignerTypes,
    witness::{Generator, Proof, WitnessError},
};
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use url::Url;

// TODO: Move to own dir, maybe w/ schema?
// TODO: Add Serde
// TODO: Support the more specific TZProfiles attestation. Requires TZProfiles specific text.

pub struct Claim {
    pub handle: String,
    pub key_type: SignerDID,
    pub tweet_id: String,
    pub tweet_url: String,
}

impl Proof for Claim {
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

pub struct Schema {
    pub handle: String,
    pub key_type: SignerDID,
    pub tweet_id: String,
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
        evidence_map.insert(
            "tweetId".to_string(),
            serde_json::Value::String(self.tweet_id.clone()),
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

        let res: TwitterResponse = client
            .get(
                Url::parse("https://api.twitter.com/2/tweets")
                    .map_err(|e| WitnessError::BadLookup(e.to_string()))?,
            )
            .query(&[
                ("ids", proof.tweet_id.to_owned()),
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

        if proof.handle.to_lowercase() != res.includes.users[0].username.to_lowercase() {
            return Err(WitnessError::BadLookup(format!(
                "unexpected handle, wanted: {} got: {}",
                proof.handle.to_lowercase(),
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
            handle: proof.handle.clone(),
            key_type: proof.key_type.clone(),
            tweet_id: proof.tweet_id.clone(),
            tweet_url: proof.tweet_url.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
