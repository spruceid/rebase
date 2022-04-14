use crate::signer::signer::{SignerError, SignerType};
use ssi::vc::LinkedDataProofOptions;

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
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    fn proof(&self, id: &str) -> Result<Option<LinkedDataProofOptions>, SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    fn valid_signature(&self, statement: &str, signature: &str, id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }
}
