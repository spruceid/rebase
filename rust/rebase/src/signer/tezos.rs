use async_trait::async_trait;
use crate::signer::signer::{SignerError, SignerType};
// use ssi::{
//     one_or_many::OneOrMany,
//     vc::{Credential, LinkedDataProofOptions, Proof, URI},
// };

pub enum Tezos {
    // TODO: Change name?
    PlainText,
}

#[async_trait(?Send)]
impl SignerType for Tezos {
    fn name(&self) -> String {
        match self {
            &Tezos::PlainText => "Tezos Address".to_string(),
        }
    }

    async fn valid_id(&self, _id: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }

    async fn as_did(&self, id: &str) -> Result<String, SignerError> {
        self.valid_id(id).await?;
        Ok(format!("did:pkh:tz:{}", id))
    }

    // TODO: Move to Signer
    // fn proof(&self, id: &str, vc: &Credential) -> Result<Option<OneOrMany<Proof>>, SignerError> {
    //     let ldpo = match self {
    //         Tezos::PlainText => LinkedDataProofOptions {
    //             verification_method: Some(URI::String(format!(
    //                 "{}#TezosMethod2021",
    //                 self.as_did(&id)?
    //             ))),
    //             ..Default::default()
    //         },
    //     };

    //     // TODO: Use VC to generate the proof.
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
