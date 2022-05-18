use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{SignerError, SignerType, DID as SignerDID};
use crate::witness::{
    signer_type::SignerTypes,
    witness::{Generator, Proof, WitnessError},
};
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use regex::Regex;
use reqwest::header::{HeaderMap, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::map::Map;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use url::Url;

// TODO: Move to own dir, maybe w/ schema?
// TODO: Add Serde
// TODO: Support the more specific TZProfiles attestation. Requires TZProfiles specific text.

pub struct Claim {
    pub gist_id: String,
    pub gist_url: String,
    pub gist_version: String,
    pub handle: String,
    pub key_type: SignerDID,
}

impl Proof for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "I am attesting that this GitHub handle {} is linked to the {} {}\n\n",
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
    pub gist_id: String,
    pub gist_url: String,
    pub gist_version: String,
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
                "GitHubVerification": "https://example.com/GitHubVerification",
                "GitHubVerificationMessage": {
                    "@id": "https://example.com/GitHubVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "gistId": "https://example.com/gistId",
                        "gistVersion":  "https://example.com/gistVersion",
                        "handle": "https://example.com/handle"
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
            "gistId".to_string(),
            serde_json::Value::String(self.gist_id.clone()),
        );

        evidence_map.insert(
            "gistVersion".to_string(),
            serde_json::Value::String(self.gist_version.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["GitHubVerificationMessage".to_string()],
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
            "sameAs": format!("https://github.com/{}", self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "GitHubVerification".to_owned(),
        ])
    }
}

pub struct ClaimGenerator {
    pub api_key: String,
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
impl Generator<Claim, Schema> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        let client = reqwest::Client::new();
        let request_url = format!("https://api.github.com/gists/{}", proof.gist_id);
        let re = Regex::new(r"^[a-zA-Z0-9]{32}$")
            .map_err(|_| WitnessError::BadLookup("could not generate gist id regex".to_string()))?;

        if !re.is_match(&proof.gist_id) {
            return Err(WitnessError::BadLookup("gist id invalid".to_string()));
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            format!("Spruce Systems").parse().map_err(|_| {
                WitnessError::BadLookup("could not generate header for lookup".to_string())
            })?,
        );

        let res: GitHubResponse = client
            .get(Url::parse(&request_url).map_err(|e| WitnessError::BadLookup(e.to_string()))?)
            .headers(headers)
            .send()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?;

        if proof.handle.to_lowercase() != res.owner.login.to_lowercase() {
            return Err(WitnessError::BadLookup(format!(
                "handle mismatch, expected: {}, got: {}",
                proof.handle.to_lowercase(),
                res.owner.login.to_lowercase()
            )));
        };

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

            match proof.parse_post(p).await {
                Err(_) => continue,
                Ok(_) => return Ok(p.to_owned()),
            };
        }

        Err(WitnessError::BadLookup(
            "Failed to find properly formatted gist".to_string(),
        ))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        statement: &str,
        signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            gist_id: proof.gist_id.clone(),
            gist_url: proof.gist_url.clone(),
            gist_version: proof.gist_version.clone(),
            handle: proof.handle.clone(),
            key_type: proof.key_type.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
