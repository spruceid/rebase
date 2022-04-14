use crate::signer::signer::{SignerError, SignerType};
use ssi::vc::LinkedDataProofOptions;

// TODO: Add EIP712 support to enable "sign_vc"
pub enum Ethereum {
    // EIP712,
    PlainText,
}

impl SignerType for Ethereum {
    fn name(&self) -> String {
        match self {
            Ethereum::PlainText => "Ed25519 Key".to_string()
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