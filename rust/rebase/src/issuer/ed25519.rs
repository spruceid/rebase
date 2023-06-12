use crate::{
    context::context_loader::context_loader,
    types::{
        defs::{Issuer, LDProof, Subject},
        error::{IssuerError, SubjectError},
    },
};
use async_trait::async_trait;
use did_web::DIDWeb;
use ed25519_dalek::{
    ed25519::signature::Signature, Keypair, PublicKey, SecretKey, Signer as InnerSigner, Verifier,
};

use hex;
use ssi::{
    jwk::{Params, JWK},
    ldp::Proof,
    one_or_many::OneOrMany,
    vc::{Credential, LinkedDataProofOptions, URI},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// TODO: Generalize over all this using SSI

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct KeyJwk {
    pub did: String,
    #[ts(type = "object")]
    pub jwk: JWK,
}

impl KeyJwk {
    pub fn new(did: &str, jwk_str: &str) -> Result<Self, IssuerError> {
        if !did.starts_with("did:key:") {
            return Err(IssuerError::Internal(format!(
                "Currently only supports did:key, got: {}",
                did
            )));
        }

        let jwk: JWK = serde_json::from_str(jwk_str)
            .map_err(|e| IssuerError::Internal(format!("deserialization error: {}", e)))?;

        Ok(KeyJwk {
            did: did.to_owned(),
            jwk,
        })
    }

    pub fn to_keypair(&self) -> Result<Keypair, SubjectError> {
        match &self.jwk.clone().params {
            Params::OKP(o) => match &o.private_key {
                Some(key) => Ok(Keypair {
                    secret: SecretKey::from_bytes(&key.0).map_err(|e| {
                        SubjectError::Validation(format!("failed to generate secret key: {}", e))
                    })?,
                    public: PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
                        SubjectError::Validation(format!("could not generate public key: {}", e))
                    })?,
                }),
                _ => Err(SubjectError::Validation(
                    "could not recover private key from jwk".to_string(),
                )),
            },
            _ => Err(SubjectError::Validation(
                "could not recover private key from jwk".to_string(),
            )),
        }
    }
}

#[async_trait(?Send)]
impl Subject for KeyJwk {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        let sig = Signature::from_bytes(
            &hex::decode(signature).map_err(|e| SubjectError::Validation(e.to_string()))?,
        )
        .map_err(|e| SubjectError::Validation(e.to_string()))?;

        let stmt = statement.as_bytes();
        let keypair = self.to_keypair()?;

        keypair
            .public
            .verify(stmt, &sig)
            .map_err(|e| SubjectError::Validation(e.to_string()))
    }
}

#[async_trait(?Send)]
impl Issuer for KeyJwk {
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError> {
        let sig = self.to_keypair()?.sign(plain_text.as_bytes());
        Ok(hex::encode(sig.to_bytes()))
    }

    async fn sign_vc(&self, credential: &mut Credential) -> Result<(), IssuerError> {
        credential.proof = self.proof(credential).await?;
        Ok(())
    }

    async fn generate_jwt(&self, credential: &Credential) -> Result<String, IssuerError> {
        let s = self.did.trim_start_matches("did:key:");
        Ok(credential
            .generate_jwt(
                Some(&self.jwk),
                &LinkedDataProofOptions {
                    checks: None,
                    created: None,
                    eip712_domain: None,
                    type_: None,
                    verification_method: Some(URI::String(format!("{}#{}", self.did()?, s))),
                    ..Default::default()
                },
                &DIDWeb,
            )
            .await
            .map_err(|e| IssuerError::Jwt(format!("Failed to generate JWT: {}", e)))?)
    }
    // proof returns the linked data proof options for a given issuer type
    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<Proof>>, IssuerError> {
        let s = self.did.trim_start_matches("did:key:");
        let lpdo = LinkedDataProofOptions {
            verification_method: Some(URI::String(format!("{}#{}", self.did()?, s))),
            ..Default::default()
        };

        Ok(Some(OneOrMany::One(
            credential
                .generate_proof(
                    &self.jwk,
                    &lpdo,
                    &DIDWeb,
                    &mut context_loader().map_err(|e| IssuerError::Vc(e.to_string()))?,
                )
                .await
                .map_err(|e| IssuerError::Proof(format!("Failed to generate LDP proof: {}", e)))?,
        )))
    }
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WebJwk {
    pub did: String,
    #[ts(type = "object")]
    pub jwk: JWK,
    pub key_name: String,
}

impl WebJwk {
    pub fn new(did: &str, jwk_str: &str, key_name: &str) -> Result<Self, IssuerError> {
        if !did.starts_with("did:web:") {
            return Err(IssuerError::Internal(format!(
                "Currently only supports did:web, got: {}",
                did
            )));
        }

        let jwk: JWK = serde_json::from_str(jwk_str)
            .map_err(|e| IssuerError::Internal(format!("deserialization error: {}", e)))?;

        Ok(WebJwk {
            did: did.to_owned(),
            jwk,
            key_name: key_name.to_owned(),
        })
    }

    pub fn to_keypair(&self) -> Result<Keypair, SubjectError> {
        match &self.jwk.clone().params {
            Params::OKP(o) => match &o.private_key {
                Some(key) => Ok(Keypair {
                    secret: SecretKey::from_bytes(&key.0).map_err(|e| {
                        SubjectError::Validation(format!("failed to generate secret key: {}", e))
                    })?,
                    public: PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
                        SubjectError::Validation(format!("could not generate public key: {}", e))
                    })?,
                }),
                _ => Err(SubjectError::Validation(
                    "could not recover private key from jwk".to_string(),
                )),
            },
            _ => Err(SubjectError::Validation(
                "could not recover private key from jwk".to_string(),
            )),
        }
    }
}

#[async_trait(?Send)]
impl Subject for WebJwk {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        let sig = Signature::from_bytes(
            &hex::decode(signature).map_err(|e| SubjectError::Validation(e.to_string()))?,
        )
        .map_err(|e| SubjectError::Validation(e.to_string()))?;

        let stmt = statement.as_bytes();
        let keypair = self.to_keypair()?;

        keypair
            .public
            .verify(stmt, &sig)
            .map_err(|e| SubjectError::Validation(e.to_string()))
    }
}

#[async_trait(?Send)]
impl Issuer for WebJwk {
    // sign takes plain text and returns the corresponding signature
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError> {
        let sig = self.to_keypair()?.sign(plain_text.as_bytes());
        Ok(hex::encode(sig.to_bytes()))
    }
    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    async fn sign_vc(&self, credential: &mut Credential) -> Result<(), IssuerError> {
        credential.proof = self.proof(credential).await?;
        Ok(())
    }
    // generate_jwt takes a VC and returns it's formatted as a JWT:
    async fn generate_jwt(&self, credential: &Credential) -> Result<String, IssuerError> {
        Ok(credential
            .generate_jwt(
                Some(&self.jwk),
                &LinkedDataProofOptions {
                    checks: None,
                    created: None,
                    eip712_domain: None,
                    type_: None,
                    verification_method: Some(URI::String(format!(
                        "{}#{}",
                        self.did()?,
                        self.key_name
                    ))),
                    ..Default::default()
                },
                &DIDWeb,
            )
            .await
            .map_err(|e| IssuerError::Jwt(format!("Failed to generate JWT: {}", e)))?)
    }
    // proof returns the linked data proof options for a given issuer type
    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<Proof>>, IssuerError> {
        let lpdo = LinkedDataProofOptions {
            verification_method: Some(URI::String(format!("{}#{}", self.did()?, self.key_name))),
            ..Default::default()
        };

        Ok(Some(OneOrMany::One(
            credential
                .generate_proof(
                    &self.jwk,
                    &lpdo,
                    &DIDWeb,
                    &mut context_loader().map_err(|e| IssuerError::Vc(e.to_string()))?,
                )
                .await
                .map_err(|e| IssuerError::Proof(format!("Failed to generate LDP proof: {}", e)))?,
        )))
    }
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(untagged)]
pub enum Ed25519 {
    Key(KeyJwk),
    Web(WebJwk),
}

#[async_trait(?Send)]
impl Subject for Ed25519 {
    fn did(&self) -> Result<String, SubjectError> {
        match self {
            Ed25519::Key(k) => k.did(),
            Ed25519::Web(w) => w.did(),
        }
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        match self {
            Ed25519::Key(k) => k.display_id(),
            Ed25519::Web(w) => w.display_id(),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        match self {
            Ed25519::Key(k) => k.valid_signature(statement, signature).await,
            Ed25519::Web(w) => w.valid_signature(statement, signature).await,
        }
    }
}

#[async_trait(?Send)]
impl Issuer for Ed25519 {
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError> {
        match self {
            Ed25519::Key(k) => k.sign(plain_text).await,
            Ed25519::Web(w) => w.sign(plain_text).await,
        }
    }
    async fn sign_vc(&self, vc: &mut Credential) -> Result<(), IssuerError> {
        match self {
            Ed25519::Key(k) => k.sign_vc(vc).await,
            Ed25519::Web(w) => w.sign_vc(vc).await,
        }
    }

    async fn generate_jwt(&self, vc: &Credential) -> Result<String, IssuerError> {
        match self {
            Ed25519::Key(k) => k.generate_jwt(vc).await,
            Ed25519::Web(w) => w.generate_jwt(vc).await,
        }
    }

    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<LDProof>>, IssuerError> {
        match self {
            Ed25519::Key(k) => k.proof(credential).await,
            Ed25519::Web(w) => w.proof(credential).await,
        }
    }
}
