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
use regex::Regex;
use reqwest::header::{HeaderMap, USER_AGENT};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::map::Map;
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
            "I am attesting that this GitHub handle {} is linked to the {} {}",
            self.handle,
            signer_type.name(),
            signer_type.statement_id()?
        ))
    }

    fn delimitor(&self) -> String {
        "\n\n".to_string()
    }
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "claim")]
pub struct Claim {
    pub gist_id: String,
    pub statement_opts: Opts,
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
    pub gist_id: String,
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
                        // "gistVersion":  "https://example.com/gistVersion",
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

#[derive(Clone, Deserialize, Serialize)]
pub struct ClaimGenerator {
    pub user_agent: String,
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
            format!("{}", self.user_agent).parse().map_err(|_| {
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

        if proof.statement_opts.handle.to_lowercase() != res.owner.login.to_lowercase() {
            return Err(WitnessError::BadLookup(format!(
                "handle mismatch, expected: {}, got: {}",
                proof.statement_opts.handle.to_lowercase(),
                res.owner.login.to_lowercase()
            )));
        };
        let s = serde_json::to_string(&res.files)
            .map_err(|e| WitnessError::SchemaError(SchemaError::Serialize(e)))?;
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
            // "Failed to find properly formatted gist".to_string(),
            format!("Failed to find files in: {}", s),
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
            // gist_version: proof.gist_version.clone(),
            handle: proof.statement_opts.handle.clone(),
            key_type: proof.statement_opts.key_type.clone(),
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
            statement_opts: Opts {
                key_type: key(),
                handle: "foo".to_owned(),
            },
            gist_id: "not_tested".to_owned(),
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
                gist_id: proof.gist_id.clone(),
                key_type: proof.statement_opts.key_type.clone(),
                handle: proof.statement_opts.handle.clone(),
                statement: statement.to_owned(),
                signature: signature.to_owned(),
            })
        }
    }

    #[tokio::test]
    async fn mock_github() {
        let sig = test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap();
        let did = mock_proof(test_eth_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_eth_did)).unwrap();

        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::GitHub, TestKey::Ed25519).unwrap();
        let did = mock_proof(test_ed25519_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_ed25519_did)).unwrap();
        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();

        let sig = test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap();
        let did = mock_proof(test_solana_did);
        let gen = MockGenerator::new(sig, || mock_proof(test_solana_did)).unwrap();
        gen.unsigned_credential(&did, &Ed25519::new(&test_ed25519_did()).unwrap())
            .await
            .unwrap();
    }
}
