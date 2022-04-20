use ssi::vc::{Credential, LinkedDataProofOptions};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignerError {
    #[error("invalid id for {signer_type:?}, {reason:?}")]
    InvalidId { signer_type: String, reason: String },
    #[error("failed to sign bytes, {0}")]
    Sign(String),

    #[error("failed to sign credential, {0}")]
    SignCredential(String),

    #[error("given message and signature did not correspond to given key")]
    InvalidSignature,

    // TODO: Remove!
    #[error("this feature is unimplemented")]
    Unimplemented,
}

pub trait SignerType {
    fn name(&self) -> String;

    fn valid_id(&self, _id: &str) -> Result<(), SignerError>;

    fn as_did(&self, id: &str) -> Result<String, SignerError>;

    // proof returns the linked data proof options for a given signer type
    fn proof(&self, id: &str) -> Result<Option<LinkedDataProofOptions>, SignerError>;

    fn valid_signature(
        &self,
        statement: &str,
        signature: &str,
        id: &str,
    ) -> Result<(), SignerError>;
}

pub trait Signer<T>
where
    T: SignerType,
{
    // TODO: Add async-trait and make these async.
    // sign takes plain text and returns the corresponding signature
    fn sign(&self, plain_text: &str) -> Result<String, SignerError>;
    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    fn sign_vc(
        &self,
        vc: &mut Credential,
        proof: Option<LinkedDataProofOptions>,
    ) -> Result<(), SignerError>;
    // id returns the identifier for the given signer, such as a public key hash.
    fn id(&self) -> String;

    fn signer_type(&self) -> T;

    fn as_did(&self) -> Result<String, SignerError> {
        self.signer_type().as_did(&self.id())
    }

    fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        self.signer_type()
            .valid_signature(statement, signature, &self.id())
    }
}
