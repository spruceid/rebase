use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ssi::{
    one_or_many::OneOrMany,
    vc::{Credential, Proof},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignerError {
    #[error("invalid signer type for {signer_type:?}, {reason:?}")]
    InvalidSignerOpts { signer_type: String, reason: String },

    #[error("invalid id for {signer_type:?}, {reason:?}")]
    InvalidId { signer_type: String, reason: String },

    #[error("failed to sign bytes, {0}")]
    Sign(String),

    #[error("failed to sign credential, {0}")]
    SignCredential(String),

    #[error("failed to verify signature for {signer_type:?}, {reason:?}")]
    InvalidSignature { signer_type: String, reason: String },

    #[error("{0}")]
    SSI(#[from] ssi::error::Error),

    // TODO: Remove!
    #[error("this feature is unimplemented")]
    Unimplemented,
}

#[async_trait(?Send)]
pub trait SignerType
where
    Self: Sized,
{
    fn name(&self) -> String;

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError>;

    fn did_id(&self) -> Result<String, SignerError>;

    fn new(t: &DID) -> Result<Self, SignerError>;

    fn did(&self) -> DID;
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EIP115 {
    pub address: String,
    pub chain_id: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum PKH {
    EIP115(Option<EIP115>),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum DID {
    PKH(PKH),
    // NOTE: Currently only supports Ed25519 keys for signing
    // Could change did::web to an enum if desired.
    Web(Option<String>),
}

impl DID {
    pub fn context(&self) -> serde_json::Value {
        match &self {
            DID::PKH(_) => serde_json::json!({
                "PKH": {
                    "address": "https://example.com/address",
                    "chain_id": "https://example.com/chain_id"
                },
            }),
            DID::Web(_) => serde_json::json!({
                "Web": "https://example.com/did_web",
            }),
        }
    }
}

impl std::fmt::Display for DID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DID::Web(o) => match o {
                Some(s) => write!(f, "ed25519 did web jwk: {}", s),
                _ => write!(f, "ed25519 did web jwk: no id set"),
            },
            DID::PKH(PKH::EIP115(o)) => match o {
                Some(s) => write!(
                    f,
                    "ethereum did pkh eip155: chain: {}, address: {}",
                    s.chain_id, s.address
                ),
                _ => write!(f, "ethereum did pkh: no id set"),
            },
        }
    }
}

#[async_trait(?Send)]
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

    fn did_id(&self) -> Result<String, SignerError> {
        self.signer_type().did_id()
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        self.signer_type()
            .valid_signature(statement, signature)
            .await
    }

    fn did(&self) -> DID {
        self.signer_type().did()
    }
}
