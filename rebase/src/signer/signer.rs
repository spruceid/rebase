use async_trait::async_trait;
use ssi::{
    one_or_many::OneOrMany,
    vc::{Credential, Proof},
};

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

    #[error("{0}")]
    SSI(#[from] ssi::error::Error),

    // TODO: Remove!
    #[error("this feature is unimplemented")]
    Unimplemented,
}

#[async_trait]
pub trait SignerType {
    fn name(&self) -> String;

    async fn valid_id(&self, _id: &str) -> Result<(), SignerError>;

    fn as_did(&self, id: &str) -> Result<String, SignerError>;

    async fn valid_signature(
        &self,
        statement: &str,
        signature: &str,
        id: &str,
    ) -> Result<(), SignerError>;
}

#[async_trait]
pub trait Signer<T>
where
    T: SignerType,
{
    // sign takes plain text and returns the corresponding signature
    async fn sign(&self, plain_text: &str) -> Result<String, SignerError>;
    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    async fn sign_vc(&self, vc: &mut Credential) -> Result<(), SignerError>;

    // id returns the identifier for the given signer, such as a public key hash.
    fn id(&self) -> String;

    fn signer_type(&self) -> T;

    // proof returns the linked data proof options for a given signer type
    async fn proof(&self, credential: &Credential)
        -> Result<Option<OneOrMany<Proof>>, SignerError>;

    fn as_did(&self) -> Result<String, SignerError> {
        self.signer_type().as_did(&self.id())
    }

    async fn valid_signature(
        &self,
        statement: String,
        signature: String,
    ) -> Result<(), SignerError>;

    // TODO: RESTORE ONCE FUTURE ISSUES ARE RESOLVED.
    /*
    async fn valid_signature(&self, statement: String, signature: String) -> Result<(), SignerError> {
        self.signer_type()
            .valid_signature(&statement, &signature, &self.id())
            .await
    }
    */
}
