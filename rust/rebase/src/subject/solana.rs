use crate::types::{defs::*, error::SubjectError};
use async_trait::async_trait;
use base58::{FromBase58, FromBase58Error};
use ed25519_dalek::{ed25519::signature::Signature as Ed25519Sig, PublicKey, Verifier};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

fn b58_err(err: FromBase58Error) -> String {
    match err {
        FromBase58Error::InvalidBase58Character(_, _) => "invalid base58 character".to_string(),
        FromBase58Error::InvalidBase58Length => "invalid base58 length".to_string(),
    }
}

// TODO: Understand where this came from.
// Noted in the did:pkh w3 docs as the network for did:pkh:solana
// but should note where / how that's discoverable to know if it's going to change
// or if it's not going to change, then note it as a magic string.
pub const SOLANA_NETWORK: &str = "4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ";

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Solana {
    pub address: String,
}

impl Solana {
    fn pubkey(&self) -> Result<PublicKey, SubjectError> {
        PublicKey::from_bytes(&self.address.from_base58().map_err(|e| {
            SubjectError::Validation(format!("failed to decode from base58: {}", b58_err(e)))
        })?)
        .map_err(|e| SubjectError::Validation(format!("failed to create from bytes: {}", e)))
    }
}

#[async_trait(?Send)]
impl Subject for Solana {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(format!(
            "did:pkh:solana:{}:{}",
            SOLANA_NETWORK, self.address
        ))
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.address.clone())
    }

    fn verification_method(&self) -> Result<String, SubjectError> {
        // NOTE: If encountering issues with this approach, use: SolanaMethod2021 instead of "controller"
        Ok(format!("{}#controller", self.did()?))
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        let sig_hex =
            hex::decode(signature).map_err(|e| SubjectError::Validation(e.to_string()))?;

        let sig = Ed25519Sig::from_bytes(&sig_hex)
            .map_err(|e| SubjectError::Validation(e.to_string()))?;

        self.pubkey()?
            .verify(statement.as_bytes(), &sig)
            .map_err(|e| SubjectError::Validation(e.to_string()))
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::test_util::util::{
        test_solana_did, test_solana_did_2, test_witness_signature, test_witness_statement,
        TestKey, TestWitness,
    };

    #[tokio::test]
    async fn test_solana() {
        let subject = &test_solana_did();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::Reddit, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Reddit, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
        subject
            .valid_signature(
                &test_witness_statement(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_solana_fail() {
        let subject = &test_solana_did();
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Said invalid signature was valid");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Said invalid signature was valid");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Said invalid signature was valid");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Said invalid signature was valid");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Said invalid signature was valid");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Said invalid signature was valid");
        }
    }

    #[tokio::test]
    async fn test_solana_bad_key() {
        let subject = &test_solana_did_2();
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::DNS, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Invalid signature permitted");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::GitHub, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Invalid signature permitted");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::Reddit, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Reddit, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Invalid signature permitted");
        }
        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::SoundCloud, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Invalid signature permitted");
        }

        if subject
            .valid_signature(
                &test_witness_statement(TestWitness::Twitter, TestKey::Solana).unwrap(),
                &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
            )
            .await
            .is_ok()
        {
            panic!("Invalid signature permitted");
        };
    }
}
