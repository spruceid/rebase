
use async_trait::async_trait;
use crate::signer::signer::{SignerError, SignerType};
// use ssi::{
//     one_or_many::OneOrMany,
//     vc::{Credential, Proof},
// };

// TODO: Add EIP712 support to enable "sign_vc"
pub enum Ethereum {
    // EIP712,
    PlainText,
}

#[async_trait(?Send)]
impl SignerType for Ethereum {
    fn name(&self) -> String {
        match self {
            Ethereum::PlainText => "Ed25519 Key".to_string(),
        }
    }

    async fn valid_id(&self, _id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    async fn as_did(&self, id: &str) -> Result<String, SignerError> {
        // TODO: IMPLEMENT
        self.valid_id(id).await?;
        Ok(format!("did:pkh:eth:{}", id))
    }

    // TODO: Move to Signer
    // fn proof(&self, id: &str, vc: &Credential) -> Result<Option<OneOrMany<Proof>>, SignerError> {
    // //     // TODO: IMPLEMENT
    //     self.valid_id(id)?;
    //     Err(SignerError::Unimplemented)
    // }

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
