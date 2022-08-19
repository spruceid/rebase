use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ssi::{
    ldp::Proof,
    one_or_many::OneOrMany,
    // vc::{Credential, Proof},
    vc::Credential,
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

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "eip155")]
pub struct EIP155 {
    pub address: String,
    pub chain_id: String,
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "solana")]
pub struct Solana {
    pub address: String,
}

// TODO: Understand where this came from.
// Noted in the did:pkh w3 docs as the network for did:pkh:solana
// but should note where / how that's discoverable to know if it's going to change
// or if it's not going to change, then note it as a magic string.
pub const SOLANA_NETWORK: &str = "4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ";

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "pkh")]
pub enum PKH {
    #[serde(rename = "eip155")]
    EIP155(Option<EIP155>),
    #[serde(rename = "solana")]
    Solana(Option<Solana>),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "did")]
pub enum DID {
    #[serde(rename = "pkh")]
    PKH(PKH),
    // NOTE: Currently only supports Ed25519 keys for signing
    // Could change did::web to an enum if desired.
    #[serde(rename = "web")]
    Web(Option<String>),
}

impl DID {
    pub fn context(&self) -> serde_json::Value {
        match &self {
            DID::PKH(PKH::EIP155(_)) => serde_json::json!({
                "pkh": {
                    "address": "https://example.com/address",
                    "chain_id": "https://example.com/chain_id"
                },
            }),
            DID::PKH(PKH::Solana(_)) => serde_json::json!({
                "pkh": {
                    "address": "https://example.com/address",
                    "network": "https://example.com/network"
                },
            }),
            DID::Web(_) => serde_json::json!({
                "web": "https://example.com/did_web",
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
            DID::PKH(PKH::EIP155(o)) => match o {
                Some(s) => write!(
                    f,
                    "ethereum did pkh eip155: chain: {}, address: {}",
                    s.chain_id, s.address
                ),
                _ => write!(f, "ethereum did pkh: no id set"),
            },
            DID::PKH(PKH::Solana(o)) => match o {
                Some(s) => write!(
                    f,
                    "solana did pkh network: {}, address: {}",
                    SOLANA_NETWORK, s.address
                ),
                _ => write!(f, "solana did pkh: no id set"),
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
    // generate_jwt takes a VC and returns it's formatted as a JWT:
    async fn generate_jwt(&self, vc: &Credential) -> Result<String, SignerError>;

    // id returns the identifier for the given signer, such as a public key hash.
    async fn id(&self) -> Result<String, SignerError>;

    async fn signer_type(&self) -> Result<T, SignerError>;

    // proof returns the linked data proof options for a given signer type
    async fn proof(&self, credential: &Credential)
        -> Result<Option<OneOrMany<Proof>>, SignerError>;

    async fn did_id(&self) -> Result<String, SignerError> {
        self.signer_type().await?.did_id()
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        self.signer_type()
            .await?
            .valid_signature(statement, signature)
            .await
    }

    async fn did(&self) -> Result<DID, SignerError> {
        Ok(self.signer_type().await?.did())
    }
}
