use crate::signer::signer::{
    SignerError, SignerType, Solana as DIDPKHSolana, DID as SignerDID, PKH as SignerPKH,
    SOLANA_NETWORK,
};
use async_trait::async_trait;

use ed25519_dalek::{ed25519::signature::Signature, PublicKey, Verifier};

use base58::{FromBase58, FromBase58Error};

fn b58_err(err: FromBase58Error) -> String {
    match err {
        FromBase58Error::InvalidBase58Character(_, _) => "invalid base58 character".to_string(),
        FromBase58Error::InvalidBase58Length => "invalid base58 length".to_string(),
    }
}

pub enum PKH {
    Solana(Option<DIDPKHSolana>),
}

pub enum DID {
    // Supported:
    // did:pkh:solana:<SOLANA_NETWORK>:<address>
    PKH(PKH),
    // Unsupported::
    // did:pkh:sol:<address>, the supported legacy PKH.
    // did:sol:<address>
}

pub enum Solana {
    DID(DID),
}

impl Solana {
    fn pubkey_err(&self, e: String) -> SignerError {
        SignerError::InvalidId {
            signer_type: self.name(),
            reason: format!("failed to build public key: {}", e.to_string()),
        }
    }

    fn pubkey(&self) -> Result<PublicKey, SignerError> {
        match self {
            Solana::DID(DID::PKH(PKH::Solana(Some(solana)))) => Ok(PublicKey::from_bytes(
                &solana.address.from_base58().map_err(|e| {
                    self.pubkey_err(format!("failed to decode from base58: {}", b58_err(e)))
                })?,
            )
            .map_err(|e| {
                self.pubkey_err(format!("failed to create from bytes: {}", e.to_string()))
            })?),
            _ => Err(SignerError::InvalidId {
                signer_type: self.name(),
                reason: format!("expected solana based signer, got: {}", self.did_id()?),
            }),
        }
    }
}

#[async_trait(?Send)]
impl SignerType for Solana {
    fn new(t: &SignerDID) -> Result<Self, SignerError> {
        // TODO: Screen for valid opts.
        match t {
            SignerDID::PKH(SignerPKH::Solana(solana)) => {
                Ok(Solana::DID(DID::PKH(PKH::Solana(solana.clone()))))
            }
            _ => Err(SignerError::InvalidSignerOpts {
                signer_type: t.to_string(),
                reason: "expected solana signer type".to_string(),
            }),
        }
    }

    fn did(&self) -> SignerDID {
        match self {
            Solana::DID(DID::PKH(PKH::Solana(solana))) => {
                SignerDID::PKH(SignerPKH::Solana(solana.clone()))
            }
        }
    }

    fn name(&self) -> String {
        "Solana Address".to_string()
    }

    fn did_id(&self) -> Result<String, SignerError> {
        match self {
            Solana::DID(DID::PKH(PKH::Solana(Some(s)))) => {
                Ok(format!("did:pkh:solana:{}:{}", SOLANA_NETWORK, s.address))
            }
            _ => Err(SignerError::InvalidId {
                signer_type: self.name(),
                reason: "expected solana based signer type".to_string(),
            }),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
        match self {
            Solana::DID(DID::PKH(PKH::Solana(Some(_)))) => {
                let sig_hex =
                    hex::decode(signature).map_err(|e| SignerError::InvalidSignature {
                        signer_type: self.name(),
                        reason: e.to_string(),
                    })?;

                let sig =
                    Signature::from_bytes(&sig_hex).map_err(|e| SignerError::InvalidSignature {
                        signer_type: self.name(),
                        reason: e.to_string(),
                    })?;

                self.pubkey()?
                    .verify(statement.as_bytes(), &sig)
                    .map_err(|e| SignerError::InvalidSignature {
                        signer_type: self.name(),
                        reason: e.to_string(),
                    })
            }

            _ => Err(SignerError::InvalidId {
                signer_type: self.name(),
                reason: "expected solana based signer type".to_string(),
            }),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::util::util::{
        test_solana_did, test_solana_did_2, test_witness_signature, test_witness_statement, TestKey, TestWitness,
    };

    #[tokio::test]
    async fn test_solana() {
        let signer_type = Solana::new(&test_solana_did()).unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Reddit, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Reddit, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_solana_fail() {
        let signer_type = Solana::new(&test_solana_did()).unwrap();
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Said invalid signature was valid"),
            Err(_) => {}
        };
    }

    #[tokio::test]
    async fn test_solana_bad_key() {
        let signer_type = Solana::new(&test_solana_did_2()).unwrap();
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Reddit, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Reddit, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
        match signer_type
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
        {
            Ok(_) => panic!("Invalid signature permitted"),
            Err(_) => {}
        };
    }
}
