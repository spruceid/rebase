// TODO: Figure out how to impl Signer<SignerTypes> for Signers
use crate::{
    signer::{
        ed25519::Ed25519DidWebJwk,
        signer::{Signer, SignerError},
    },
    witness::{signer_type::SignerTypes, witness::WitnessError},
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ssi::{ldp::Proof, one_or_many::OneOrMany, vc::Credential};

#[async_trait(?Send)]
trait SignerConfig {
    // TODO: Find a way to enforce this here without struggling with the typechecker.
    // async fn to_signer<T: SignerType, S: Signer<T>>(&self) -> Result<S, WitnessError>;
    async fn to_signer_types(&self) -> Result<SignerTypes, WitnessError>;
}

#[derive(Deserialize, Serialize)]
pub struct Ed25519DidWebJwkOpts {
    key: String,
    did: String,
    name: String,
}

impl Ed25519DidWebJwkOpts {
    async fn to_signer(&self) -> Result<Ed25519DidWebJwk, WitnessError> {
        Ok(Ed25519DidWebJwk::new(&self.did, &self.key, &self.name).await?)
    }
}

#[async_trait(?Send)]
impl SignerConfig for Ed25519DidWebJwkOpts {
    async fn to_signer_types(&self) -> Result<SignerTypes, WitnessError> {
        Ok(SignerTypes::Ed25519(
            self.to_signer().await?.signer_type().await?,
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub enum Signers {
    #[serde(rename = "ed25519_did_web_jwk")]
    Ed25519DidWebJwk(Ed25519DidWebJwkOpts),
}

#[async_trait(?Send)]
impl Signer<SignerTypes> for Signers {
    async fn sign(&self, plain_text: &str) -> Result<String, SignerError> {
        match self {
            Signers::Ed25519DidWebJwk(c) => {
                c.to_signer()
                    .await
                    .map_err(|e| SignerError::InvalidSignerOpts {
                        signer_type: "ed25519".to_string(),
                        reason: e.to_string(),
                    })?
                    .sign(plain_text)
                    .await
            }
        }
    }

    async fn sign_vc(&self, vc: &mut Credential) -> Result<(), SignerError> {
        match self {
            Signers::Ed25519DidWebJwk(c) => {
                c.to_signer()
                    .await
                    .map_err(|e| SignerError::InvalidSignerOpts {
                        signer_type: "ed25519".to_string(),
                        reason: e.to_string(),
                    })?
                    .sign_vc(vc)
                    .await
            }
        }
    }

    async fn generate_jwt(&self, vc: &Credential) -> Result<String, SignerError> {
        match self {
            Signers::Ed25519DidWebJwk(c) => {
                c.to_signer()
                    .await
                    .map_err(|e| SignerError::InvalidSignerOpts {
                        signer_type: "ed25519".to_string(),
                        reason: e.to_string(),
                    })?
                    .generate_jwt(vc)
                    .await
            }
        }
    }

    async fn id(&self) -> Result<String, SignerError> {
        match self {
            Signers::Ed25519DidWebJwk(c) => {
                c.to_signer()
                    .await
                    .map_err(|e| SignerError::InvalidSignerOpts {
                        signer_type: "ed25519".to_string(),
                        reason: e.to_string(),
                    })?
                    .id()
                    .await
            }
        }
    }

    async fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        match self {
            Signers::Ed25519DidWebJwk(c) => {
                c.to_signer_types()
                    .await
                    .map_err(|e| SignerError::InvalidSignerOpts {
                        signer_type: "ed25519".to_string(),
                        reason: e.to_string(),
                    })
            }
        }
    }

    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<Proof>>, SignerError> {
        match self {
            Signers::Ed25519DidWebJwk(c) => {
                c.to_signer()
                    .await
                    .map_err(|e| SignerError::InvalidSignerOpts {
                        signer_type: "ed25519".to_string(),
                        reason: e.to_string(),
                    })?
                    .proof(credential)
                    .await
            }
        }
    }
}
