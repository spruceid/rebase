use crate::signer::signer::{Signer, SignerError, SignerType};
use async_trait::async_trait;
use did_web::DIDWeb;
// use ed25519_dalek::{Signer as InnerSigner};
use ssi::{
    jwk::JWK,
    one_or_many::OneOrMany,
    vc::{Credential, LinkedDataProofOptions, Proof, URI},
};

#[derive(Clone)]
pub enum SignerTypes {
    // TODO: Change name?
    DIDWebJWK,
}

pub struct Ed25519 {
    pub id: String,
    pub key: JWK,
    pub key_name: String,
    signer_type: SignerTypes,
}

impl Ed25519 {
    pub async fn new(id: String, key: JWK, key_name: String, signer_type: SignerTypes) -> Result<Self, SignerError> {
        signer_type.valid_id(&id).await?;
        Ok(Ed25519 {
            id,
            key,
            key_name,
            signer_type,
        })
    }
}

#[async_trait(?Send)]
impl Signer<SignerTypes> for Ed25519 {
    // TODO: IMPL
    async fn sign(&self, _plain_text: &str) -> Result<String, SignerError> {
        Err(SignerError::Unimplemented)
    }

    async fn sign_vc(&self, vc: &mut Credential) -> Result<(), SignerError> {
        vc.proof = self.proof(vc).await?;
        Ok(())
    }

    async fn proof(&self, vc: &Credential) -> Result<Option<OneOrMany<Proof>>, SignerError> {
        let lpdo = match self.signer_type {
            SignerTypes::DIDWebJWK => LinkedDataProofOptions {
                verification_method: Some(URI::String(format!(
                    "{}#{}",
                    self.signer_type.as_did(&self.id).await?,
                    self.key_name
                ))),
                ..Default::default()
            }
        };

        Ok(Some(OneOrMany::One(
            vc.generate_proof(&self.key, &lpdo, &DIDWeb).await?,
        )))
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn signer_type(&self) -> SignerTypes {
        self.signer_type.clone()
    }
}

#[async_trait(?Send)]
impl SignerType for SignerTypes {
    fn name(&self) -> String {
        match self {
            SignerTypes::DIDWebJWK => "Ed25519 Web Key".to_string(),
        }
    }

    async fn valid_id(&self, _id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        // Err(SignerError::Unimplemented)
        Ok(())
    }

    async fn as_did(&self, id: &str) -> Result<String, SignerError> {
        self.valid_id(id).await?;
        match self {
            SignerTypes::DIDWebJWK => Ok(id.to_owned()),
        }
    }

    async fn valid_signature(
        &self,
        _statement: &str,
        _signature: &str,
        id: &str,
    ) -> Result<(), SignerError> {
        self.valid_id(id).await?;
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }
}
