use crate::types::{
    defs::{resolve_key, DIDKey, DIDWeb, Subject, JWK},
    error::SubjectError,
};
use async_trait::async_trait;
use ed25519_dalek::{ed25519::signature::Signature, PublicKey, Verifier};
use hex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ssi::jwk::Params;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Ed25519Jwk {
    pub did: String,
    pub key_name: String,
}

impl Ed25519Jwk {
    pub fn new(did: &str, key_name: &str) -> Result<Self, SubjectError> {
        Ok(Ed25519Jwk {
            did: did.to_owned(),
            key_name: key_name.to_owned(),
        })
    }

    pub async fn jwk(&self) -> Result<JWK, SubjectError> {
        if !self.did.starts_with("did:web:") && !self.did.starts_with("did:key:") {
            return Err(SubjectError::Did(format!(
                "Currently only supports ed25519 keys as did:web or did:key, got: {}",
                self.did
            )));
        }

        let full_did = format!("{}#{}", self.did, self.key_name);
        if self.did.starts_with("did:key:") {
            let resolver = DIDKey {};
            resolve_key(&full_did, &resolver).await.map_err(|e| {
                SubjectError::Validation(format!("Could not build JWK from DID: {}", e))
            })
        } else if self.did.starts_with("did:web:") {
            let resolver = DIDWeb {};
            resolve_key(&full_did, &resolver).await.map_err(|e| {
                SubjectError::Validation(format!("Could not build JWK from DID: {}", e))
            })
        } else {
            Err(SubjectError::Validation(format!(
                "Delegate DID must be of did:web or did:key, got {}",
                self.did
            )))
        }
    }
}

#[async_trait(?Send)]
impl Subject for Ed25519Jwk {
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
        let jwk = self.jwk().await?;
        let pk = match &jwk.params {
            Params::OKP(o) => Ok(PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
                SubjectError::Validation(format!("could not generate public key: {}", e))
            })?),
            _ => Err(SubjectError::Validation(
                "could not recover public key from jwk".to_string(),
            )),
        }?;

        let statement = statement.as_bytes();
        let signature = Signature::from_bytes(
            &hex::decode(signature).map_err(|e| SubjectError::Validation(e.to_string()))?,
        )
        .map_err(|e| SubjectError::Validation(e.to_string()))?;

        pk.verify(statement, &signature)
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

// NOTE: Tests for this file can be found in issuer/ed25519
// There the function test_did_kepair from test_util is tested as
// both issuer and subject.
