use crate::signer::signer::{SignerError, SignerType};
use ssi::vc::{LinkedDataProofOptions, URI};

pub enum Ed25519 {
    // TODO: Change name?
    DIDWebJWK,
}

impl SignerType for Ed25519 {
    fn name(&self) -> String {
        match self {
            Ed25519::DIDWebJWK => "Ed25519 Key".to_string(),
        }
    }

    fn valid_id(&self, id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    fn as_did(&self, id: &str) -> Result<String, SignerError> {
        self.valid_id(id)?;
        match self {
            Ed25519::DIDWebJWK => {
                Ok(id.to_owned())
            }
        }
    }

    fn proof(&self, id: &str) -> Result<Option<LinkedDataProofOptions>, SignerError> {
        self.valid_id(id)?;
        match self {
            Ed25519::DIDWebJWK => Ok(Some(LinkedDataProofOptions {
                verification_method: Some(URI::String(format!("{}#controller", self.as_did(&id)?))),
                ..Default::default()
            })),
        }
    }

    fn valid_signature(
        &self,
        statement: &str,
        signature: &str,
        id: &str,
    ) -> Result<(), SignerError> {
        self.valid_id(id)?;
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }
}
