use crate::signer::signer::{SignerError, SignerType, DID as SignerDID, EIP115, PKH as SignerPKH};
use async_trait::async_trait;

// TODO: Break EIP115 into own file to use with other chains.
pub enum PKH {
    EIP115(Option<EIP115>),
}

pub enum DID {
    PKH(PKH),
}

pub enum Ethereum {
    DID(DID),
}

#[async_trait(?Send)]
impl SignerType for Ethereum {
    fn new(t: &SignerDID) -> Result<Self, SignerError> {
        // TODO: Screen for valid opts.
        match t {
            SignerDID::PKH(SignerPKH::EIP115(o)) => {
                Ok(Ethereum::DID(DID::PKH(PKH::EIP115(o.clone()))))
            }
            _ => Err(SignerError::InvalidSignerOpts {
                signer_type: t.to_string(),
                reason: "expected ethereum signer type".to_string(),
            }),
        }
    }

    fn did(&self) -> SignerDID {
        match self {
            Ethereum::DID(DID::PKH(PKH::EIP115(o))) => SignerDID::PKH(SignerPKH::EIP115(o.clone())),
        }
    }

    fn name(&self) -> String {
        "Ethereum Address".to_string()
    }

    fn did_id(&self) -> Result<String, SignerError> {
        match self {
            Ethereum::DID(DID::PKH(PKH::EIP115(Some(o)))) => {
                Ok(format!("did:pkh:eip115:{}:{}", o.chain_id, o.address))
            }
            _ => Err(SignerError::InvalidId {
                signer_type: self.name(),
                reason: "expected ethereum based signer type".to_string(),
            }),
        }
    }

    async fn valid_signature(&self, _statement: &str, _signature: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        Err(SignerError::Unimplemented)
    }
}

// TODO: Add EIP712 support to enable "sign_vc"
// Will need for signer
pub enum Method {
    EIP712,
    PlainText,
}
