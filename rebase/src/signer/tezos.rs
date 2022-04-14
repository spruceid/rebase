use crate::signer::signer::{SignerError, SignerType};
use ssi::vc::LinkedDataProofOptions;

pub enum Tezos {
    // TODO: Change name?
    PlainText,
}

impl SignerType for Tezos {
    fn name(&self) -> String {
        match self {
            &Tezos::PlainText => "Tezos Address".to_string()
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

