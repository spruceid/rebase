use crate::types::{defs::Subject, error::SubjectError};
use async_trait::async_trait;
use ed25519_dalek::{ed25519::signature::Signature, PublicKey, Verifier};
use hex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ssi::jwk::Base64urlUInt;
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[serde(rename = "did_web")]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DidWeb {
    pub did: String,
    pub key_name: String,
}

impl DidWeb {
    pub async fn pubkey(&self) -> Result<PublicKey, SubjectError> {
        match self.did.strip_prefix("did:web:") {
            Some(u) => {
                let client = reqwest::Client::new();
                let request_url = format!("https://{}/.well-known/did.json", u);
                let res: Res = client
                    .get(
                        Url::parse(&request_url)
                            .map_err(|e| SubjectError::Validation(e.to_string()))?,
                    )
                    .send()
                    .await
                    .map_err(|e| {
                        SubjectError::Validation(format!("failed to retrieve public key: {}", e))
                    })?
                    .json()
                    .await
                    .map_err(|e| {
                        SubjectError::Validation(format!("invalid public key format: {}", e))
                    })?;

                if res.verification_method.is_empty() {
                    return Err(SubjectError::Validation(
                        "no verifications found in did document".to_string(),
                    ));
                };

                let b = Base64urlUInt::try_from(res.verification_method[0].key.x.clone()).map_err(
                    |e| SubjectError::Validation(format!("failed to decode public key: {}", e)),
                )?;

                Ok(PublicKey::from_bytes(&b.0).map_err(|e| {
                    SubjectError::Validation(format!("failed to create from bytes: {}", e))
                })?)
            }
            None => Err(SubjectError::Validation(format!(
                "Unexpected did web format: {}",
                self.did
            ))),
        }
    }
}

#[async_trait(?Send)]
impl Subject for DidWeb {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(self.did.clone())
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        let s = self.did.trim_start_matches("did:web:");
        Ok(s.to_owned())
    }

    fn verification_method(&self) -> Result<String, SubjectError> {
        Ok(format!("{}#{}", &self.did, &self.key_name))
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        let sig = Signature::from_bytes(
            &hex::decode(signature).map_err(|e| SubjectError::Validation(e.to_string()))?,
        )
        .map_err(|e| SubjectError::Validation(e.to_string()))?;

        let stmt = statement.as_bytes();
        let pubkey = self.pubkey().await?;

        pubkey
            .verify(stmt, &sig)
            .map_err(|e| SubjectError::Validation(e.to_string()))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Res {
    #[serde(rename = "@context")]
    // pub context: Vec<Context>,
    pub context: String,
    pub id: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub controller: String,
    #[serde(rename = "publicKeyJwk")]
    pub key: VerificationPubKey,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationPubKey {
    pub kty: String,
    pub crv: String,
    pub x: String,
}

#[derive(Deserialize, Serialize)]
pub struct ContextKey {
    #[serde(rename = "Ed25519VerificationKey2018")]
    pub verification_key: String,
    #[serde(rename = "publicKeyJwk")]
    pub public_key: ContextPubKey,
}

#[derive(Deserialize, Serialize)]
pub struct ContextPubKey {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    String(String),
    Struct(ContextKey),
}
