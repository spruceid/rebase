use crate::signer::signer::{Signer, SignerError, SignerType};
use did_web::DIDWeb;
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
    signer_type: SignerTypes,
}

impl Ed25519 {
    pub fn new(id: String, key: JWK, signer_type: SignerTypes) -> Result<Self, SignerError> {
        signer_type.valid_id(&id)?;
        Ok(Ed25519 {
            id,
            key,
            signer_type,
        })
    }
}

impl Signer<SignerTypes> for Ed25519 {
    // TODO: IMPL
    fn sign(&self, plain_text: &str) -> Result<String, SignerError> {
        Err(SignerError::Unimplemented)
    }

    // TODO: IMPL
    fn sign_vc(&self, vc: &mut Credential) -> Result<(), SignerError> {
        Err(SignerError::Unimplemented)
    }

    fn proof(&self, vc: &Credential) -> Result<Option<OneOrMany<Proof>>, SignerError> {
        let lpdo = match self.signer_type {
            SignerTypes::DIDWebJWK => LinkedDataProofOptions {
                verification_method: Some(URI::String(format!(
                    "{}#controller",
                    self.signer_type.as_did(&self.id)?
                ))),
                ..Default::default()
            },
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

impl SignerType for SignerTypes {
    fn name(&self) -> String {
        match self {
            SignerTypes::DIDWebJWK => "Ed25519 Key".to_string(),
        }
    }

    fn valid_id(&self, _id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    fn as_did(&self, id: &str) -> Result<String, SignerError> {
        self.valid_id(id)?;
        match self {
            SignerTypes::DIDWebJWK => Ok(id.to_owned()),
        }
    }

    fn valid_signature(
        &self,
        _statement: &str,
        _signature: &str,
        id: &str,
    ) -> Result<(), SignerError> {
        self.valid_id(id)?;
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }
}
