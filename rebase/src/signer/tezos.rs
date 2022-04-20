use crate::signer::signer::{SignerError, SignerType};
use ssi::vc::{LinkedDataProofOptions, URI};

pub enum Tezos {
    // TODO: Change name?
    PlainText,
}

impl SignerType for Tezos {
    fn name(&self) -> String {
        match self {
            &Tezos::PlainText => "Tezos Address".to_string(),
        }
    }

    fn valid_id(&self, _id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    fn as_did(&self, id: &str) -> Result<String, SignerError> {
        self.valid_id(id)?;
        Ok(format!("did:pkh:tz:{}", id))
    }

    fn proof(&self, id: &str) -> Result<Option<LinkedDataProofOptions>, SignerError> {
        match self {
            Tezos::PlainText => Ok(Some(LinkedDataProofOptions {
                verification_method: Some(URI::String(format!(
                    "{}#TezosMethod2021",
                    self.as_did(&id)?
                ))),
                ..Default::default()
            })),
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
