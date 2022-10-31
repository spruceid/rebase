use crate::signer::{
    ed25519::Ed25519,
    ethereum::{Ethereum, DID as EthDID, PKH as EthPKH},
    signer::{SignerError, SignerType, DID as SignerDID, PKH as SignerPKH},
    solana::{Solana, DID as SolanaDID, PKH as SolanaPKH},
};
use crate::witness::witness::WitnessError;

use async_trait::async_trait;

pub enum SignerTypes {
    Ed25519(Ed25519),
    Ethereum(Ethereum),
    Solana(Solana),
}

#[async_trait(?Send)]
impl SignerType for SignerTypes {
    fn name(&self) -> String {
        match self {
            SignerTypes::Ed25519(x) => x.name(),
            SignerTypes::Ethereum(x) => x.name(),
            SignerTypes::Solana(x) => x.name(),
        }
    }

    fn did_id(&self) -> Result<String, SignerError> {
        match self {
            SignerTypes::Ed25519(x) => x.did_id(),
            SignerTypes::Ethereum(x) => x.did_id(),
            SignerTypes::Solana(x) => x.did_id(),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        match self {
            SignerTypes::Ed25519(x) => x.valid_signature(statement, signature).await,
            SignerTypes::Ethereum(x) => x.valid_signature(statement, signature).await,
            SignerTypes::Solana(x) => x.valid_signature(statement, signature).await,
        }
    }

    fn new(t: &SignerDID) -> Result<Self, SignerError> {
        match t {
            // NOTE: Currently only supports Ed25519 keys for signing
            // Could change did::web to an enum if desired.
            SignerDID::Web(o) => Ok(SignerTypes::Ed25519(Ed25519::DIDWebJWK(o.clone()))),
            SignerDID::PKH(pkh) => match pkh {
                SignerPKH::EIP155(o) => Ok(SignerTypes::Ethereum(Ethereum::DID(EthDID::PKH(
                    EthPKH::EIP155(o.clone()),
                )))),
                SignerPKH::Solana(o) => Ok(SignerTypes::Solana(Solana::DID(SolanaDID::PKH(
                    SolanaPKH::Solana(o.clone()),
                )))),
            },
        }
    }

    fn did(&self) -> SignerDID {
        match self {
            SignerTypes::Ed25519(x) => x.did(),
            SignerTypes::Ethereum(x) => x.did(),
            SignerTypes::Solana(x) => x.did(),
        }
    }
}

impl SignerTypes {
    // Used in statement generation. Can reformat internal represetation this way
    pub fn statement_id(&self) -> Result<String, WitnessError> {
        match self {
            SignerTypes::Ed25519(x) => match x.did_id()?.split(":").last() {
                Some(s) => Ok(s.to_owned()),
                None => Err(WitnessError::SignerError(SignerError::InvalidId {
                    signer_type: x.name(),
                    reason: "failed to generate did id".to_owned(),
                })),
            },
            SignerTypes::Ethereum(x) => match x.did_id()?.split(":").last() {
                Some(s) => Ok(s.to_owned()),
                None => Err(WitnessError::SignerError(SignerError::InvalidId {
                    signer_type: x.name(),
                    reason: "failed to generate did id".to_owned(),
                })),
            },
            SignerTypes::Solana(x) => match x.did_id()?.split(":").last() {
                Some(s) => Ok(s.to_owned()),
                None => Err(WitnessError::SignerError(SignerError::InvalidId {
                    signer_type: x.name(),
                    reason: "failed to generate did id".to_owned(),
                })),
            },
        }
    }
}
