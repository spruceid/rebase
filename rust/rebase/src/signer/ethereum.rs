use crate::signer::signer::{SignerError, SignerType, DID as SignerDID, EIP155, PKH as SignerPKH};
use async_trait::async_trait;
use hex::FromHex;
use k256::{
    ecdsa::{
        recoverable::{Id, Signature},
        signature::Signature as S,
        Signature as Sig,
    },
    elliptic_curve::sec1::ToEncodedPoint,
};
use sha3::{Digest, Keccak256};

// TODO: Break EIP155 into own file to use with other chains.
pub enum PKH {
    EIP155(Option<EIP155>),
}

pub enum DID {
    PKH(PKH),
}

pub enum Ethereum {
    DID(DID),
}
// TODO: Add EIP712 support to enable "sign_vc"?
// Will need for signer
pub enum Method {
    EIP712,
    PlainText,
}

#[async_trait(?Send)]
impl SignerType for Ethereum {
    fn new(t: &SignerDID) -> Result<Self, SignerError> {
        // TODO: Screen for valid opts.
        match t {
            SignerDID::PKH(SignerPKH::EIP155(o)) => {
                Ok(Ethereum::DID(DID::PKH(PKH::EIP155(o.clone()))))
            }
            _ => Err(SignerError::InvalidSignerOpts {
                signer_type: t.to_string(),
                reason: "expected ethereum signer type".to_string(),
            }),
        }
    }

    fn did(&self) -> SignerDID {
        match self {
            Ethereum::DID(DID::PKH(PKH::EIP155(o))) => SignerDID::PKH(SignerPKH::EIP155(o.clone())),
        }
    }

    fn name(&self) -> String {
        "Ethereum Address".to_string()
    }

    fn did_id(&self) -> Result<String, SignerError> {
        match self {
            Ethereum::DID(DID::PKH(PKH::EIP155(Some(o)))) => {
                Ok(format!("did:pkh:eip155:{}:{}", o.chain_id, o.address))
            }
            _ => Err(SignerError::InvalidId {
                signer_type: self.name(),
                reason: "expected ethereum based signer type".to_string(),
            }),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        // TODO: IMPLEMENT
        match self {
            // NOTE: THIS ASSUMES EIP191 SIGNING.
            // TODO: Call this out in the type system?
            Ethereum::DID(DID::PKH(PKH::EIP155(Some(o)))) => {
                let statement: Vec<u8> = format!(
                    "\x19Ethereum Signed Message:\n{}{}",
                    statement.as_bytes().len(),
                    statement
                )
                .into();
                let signature =
                    <[u8; 65]>::from_hex(signature.trim_start_matches("0x")).map_err(|e| {
                        SignerError::InvalidSignature {
                            signer_type: self.name(),
                            reason: format!("could not marshal signature to hex: {}", e),
                        }
                    })?;

                let pk = Signature::new(
                    &Sig::from_bytes(&signature[..64]).map_err(|e| {
                        SignerError::InvalidSignature {
                            signer_type: self.name(),
                            reason: format!("could not process signature to recover key: {}", e),
                        }
                    })?,
                    Id::new(&signature[64] % 27).map_err(|e| SignerError::InvalidSignature {
                        signer_type: self.name(),
                        reason: format!("could not process signature to recover key: {}", e),
                    })?,
                )
                .map_err(|e| SignerError::InvalidSignature {
                    signer_type: self.name(),
                    reason: format!("could not recover key: {}", e),
                })?
                .recover_verify_key(&statement)
                .map_err(|e| SignerError::InvalidSignature {
                    signer_type: self.name(),
                    reason: format!("could not process statement to recover key: {}", e),
                })?;

                let address =
                    <[u8; 20]>::from_hex(&o.address.trim_start_matches("0x")).map_err(|e| {
                        SignerError::InvalidSignature {
                            signer_type: self.name(),
                            reason: format!("could not marshal address to hex: {}", e),
                        }
                    })?;

                if Keccak256::default()
                    .chain(&pk.to_encoded_point(false).as_bytes()[1..])
                    .finalize()[12..]
                    != address
                {
                    Err(SignerError::InvalidSignature {
                        signer_type: self.name(),
                        reason: "signature mismatch".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Err(SignerError::InvalidId {
                signer_type: self.name(),
                reason: "expected ethereum based signer type".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util::util::{
        test_eth_did, test_witness_signature, test_witness_statement, TestKey, TestWitness,
    };

    #[tokio::test]
    async fn test_eth() {
        let signer_type = Ethereum::new(&test_eth_did()).unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_eth_fail() {
        let signer_type = Ethereum::new(&test_eth_did()).unwrap();
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Eth).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
    }
}
