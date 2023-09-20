use crate::{
    content::same_controller_assertion::SameControllerAssertionContent as Ctnt,
    proof::same_controller_assertion::SameControllerAssertionProof as Prf,
    statement::same_controller_assertion::SameControllerAssertionStatement as Stmt,
    types::{
        defs::{Flow, Instructions, Issuer, Proof, Statement, StatementResponse, Subject},
        error::FlowError,
    },
};

use async_trait::async_trait;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SameControllerAssertionFlow {}

#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf> for SameControllerAssertionFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions {
            statement: "Please enter both of the signers you wish to link along with what type of signer they are".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Please sign the presented statement with the signers entered in the previous step in the same order as provided".to_string(),
            witness: "Send the signatures and signer information to the witness".to_string(),
            witness_schema: schema_for!(Prf)
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &Stmt,
        _issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, _issuer: &I) -> Result<Ctnt, FlowError> {
        let s = proof.statement.generate_statement()?;
        proof
            .statement
            .id1
            .valid_signature(&s, &proof.signature1)
            .await?;

        proof
            .statement
            .id2
            .valid_signature(&s, &proof.signature2)
            .await?;

        // NOTE: The passed signature is discarded internally, using both found in proof.
        Ok(proof.to_content(&s, &proof.signature1)?)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_util::util::{
            test_did_keypair, test_ed25519_did, test_ed25519_did_2, test_eth_did, test_eth_did_2,
            test_solana_did, test_solana_did_2, test_witness_signature, MockIssuer, TestKey,
            TestWitness, TEST_2KEY_ED25519_SIG_1, TEST_2KEY_ED25519_SIG_2, TEST_2KEY_ETH_SIG_1,
            TEST_2KEY_ETH_SIG_2, TEST_2KEY_SOLANA_SIG_1, TEST_2KEY_SOLANA_SIG_2,
        },
        types::enums::subject::Subjects,
    };

    async fn mock_proof(
        key_1: fn() -> Subjects,
        key_2: fn() -> Subjects,
        sig_1: &str,
        sig_2: &str,
    ) -> Result<Prf, FlowError> {
        Ok(Prf {
            statement: Stmt {
                id1: key_1(),
                id2: key_2(),
            },
            signature1: sig_1.to_owned(),
            signature2: sig_2.to_owned(),
        })
    }

    #[tokio::test]
    async fn test_eth_claim() {
        let issuer = MockIssuer {};
        // The valid case.
        let p = mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_1,
            TEST_2KEY_ETH_SIG_2,
        )
        .await
        .unwrap();

        let flow = SameControllerAssertionFlow {};

        flow.unsigned_credential(&p, &test_eth_did(), &issuer)
            .await
            .unwrap();

        // Swapped signatures.
        let p = mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_2,
            TEST_2KEY_ETH_SIG_1,
        )
        .await
        .unwrap();

        match flow.unsigned_credential(&p, &test_eth_did(), &issuer).await {
            Err(_) => {}
            Ok(_) => panic!("Reversed signatures were incorrectly validated!"),
        }

        // Swapped keys.
        let p = mock_proof(
            test_eth_did_2,
            test_eth_did,
            TEST_2KEY_ETH_SIG_1,
            TEST_2KEY_ETH_SIG_2,
        )
        .await
        .unwrap();

        match flow.unsigned_credential(&p, &test_eth_did(), &issuer).await {
            Err(_) => {}
            Ok(_) => panic!("Reversed keys were incorrectly validated!"),
        }

        // Unrelated signatures one of three.
        let p = mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_1,
            &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
        )
        .await
        .unwrap();

        match flow.unsigned_credential(&p, &test_eth_did(), &issuer).await {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_2 was incorrectly validated!"),
        }

        // two of three
        let p = mock_proof(
            test_eth_did,
            test_eth_did_2,
            &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            TEST_2KEY_ETH_SIG_2,
        )
        .await
        .unwrap();

        match flow.unsigned_credential(&p, &test_eth_did(), &issuer).await {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_1 was incorrectly validated!"),
        }

        // three of three
        let p = mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_2,
            &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
        )
        .await
        .unwrap();
        match flow.unsigned_credential(&p, &test_eth_did(), &issuer).await {
            Err(_) => {}
            Ok(_) => panic!("Invalid signatures in both signatures were incorrectly validated!"),
        }
    }

    #[tokio::test]
    async fn test_ed25519_claim() {
        let issuer = MockIssuer {};
        let flow = SameControllerAssertionFlow {};

        // The valid case.
        let p = mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_1,
            TEST_2KEY_ED25519_SIG_2,
        )
        .await
        .unwrap();

        flow.unsigned_credential(&p, &test_ed25519_did(), &issuer)
            .await
            .unwrap();

        // Swapped signatures.
        let p = mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_2,
            TEST_2KEY_ED25519_SIG_1,
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_ed25519_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed signatures were incorrectly validated!"),
        }

        // Swapped keys.
        let p = mock_proof(
            test_ed25519_did_2,
            test_ed25519_did,
            TEST_2KEY_ED25519_SIG_1,
            TEST_2KEY_ED25519_SIG_2,
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_ed25519_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed keys were incorrectly validated!"),
        }

        // Unrelated signatures one of three.
        let p = mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_1,
            &test_witness_signature(TestWitness::DNS, TestKey::Ed25519).unwrap(),
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_ed25519_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_2 was incorrectly validated!"),
        }

        // two of three
        let p = mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            &test_witness_signature(TestWitness::GitHub, TestKey::Ed25519).unwrap(),
            TEST_2KEY_ED25519_SIG_2,
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_ed25519_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_1 was incorrectly validated!"),
        }

        // three of three
        let p = mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_2,
            &test_witness_signature(TestWitness::Twitter, TestKey::Ed25519).unwrap(),
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_ed25519_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signatures in both signatures were incorrectly validated!"),
        }
    }

    #[tokio::test]
    async fn test_solana_claim() {
        let flow = SameControllerAssertionFlow {};
        let issuer = MockIssuer {};

        // The valid case.
        let p = mock_proof(
            test_solana_did,
            test_solana_did_2,
            TEST_2KEY_SOLANA_SIG_1,
            TEST_2KEY_SOLANA_SIG_2,
        )
        .await
        .unwrap();

        flow.unsigned_credential(&p, &test_solana_did(), &issuer)
            .await
            .unwrap();

        // Swapped signatures.
        let p = mock_proof(
            test_solana_did,
            test_solana_did_2,
            TEST_2KEY_SOLANA_SIG_2,
            TEST_2KEY_SOLANA_SIG_1,
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_solana_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed signatures were incorrectly validated!"),
        }

        // Swapped keys.
        let p = mock_proof(
            test_solana_did_2,
            test_solana_did,
            TEST_2KEY_SOLANA_SIG_1,
            TEST_2KEY_SOLANA_SIG_2,
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_solana_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed keys were incorrectly validated!"),
        }

        // Unrelated signatures one of three.
        let p = mock_proof(
            test_solana_did,
            test_solana_did_2,
            TEST_2KEY_SOLANA_SIG_1,
            &test_witness_signature(TestWitness::DNS, TestKey::Solana).unwrap(),
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_solana_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_2 was incorrectly validated!"),
        }

        // two of three
        let p = mock_proof(
            test_solana_did,
            test_solana_did_2,
            &test_witness_signature(TestWitness::GitHub, TestKey::Solana).unwrap(),
            TEST_2KEY_SOLANA_SIG_2,
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_solana_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_1 was incorrectly validated!"),
        }

        // three of three
        let p = mock_proof(
            test_solana_did,
            test_solana_did_2,
            TEST_2KEY_SOLANA_SIG_2,
            &test_witness_signature(TestWitness::Twitter, TestKey::Solana).unwrap(),
        )
        .await
        .unwrap();

        match flow
            .unsigned_credential(&p, &test_solana_did(), &issuer)
            .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signatures in both signatures were incorrectly validated!"),
        }
    }

    #[tokio::test]
    async fn mock_same_controller_on_the_fly() {
        let i = MockIssuer {};
        let flow = SameControllerAssertionFlow {};
        let (subj1, iss1) = test_did_keypair().await.unwrap();
        let (subj2, iss2) = test_did_keypair().await.unwrap();
        let (_, iss3) = test_did_keypair().await.unwrap();

        let ver_stmt1 = Stmt {
            id1: subj1.clone(),
            id2: subj2.clone(),
        };

        let statement = ver_stmt1.generate_statement().unwrap();
        let signature1 = iss1.sign(&statement).await.unwrap();
        let signature2 = iss2.sign(&statement).await.unwrap();

        let p = Prf {
            statement: Stmt {
                id1: subj1.clone(),
                id2: subj2.clone(),
            },
            signature1: signature1.to_owned(),
            signature2: signature2.to_owned(),
        };

        flow.unsigned_credential(&p, &subj1, &i).await.unwrap();

        let p = Prf {
            statement: Stmt {
                id1: subj1.clone(),
                id2: subj2.clone(),
            },
            signature1: signature2.to_owned(),
            signature2: signature1.to_owned(),
        };

        if flow.unsigned_credential(&p, &subj1, &i).await.is_ok() {
            panic!("Approved first bad signature");
        };

        let signature3 = iss3.sign(&statement).await.unwrap();

        let p = Prf {
            statement: Stmt {
                id1: subj1.clone(),
                id2: subj2.clone(),
            },
            signature1: signature1.to_owned(),
            signature2: signature3.to_owned(),
        };

        if flow.unsigned_credential(&p, &subj2, &i).await.is_ok() {
            panic!("Approved second bad signature");
        };
    }
}
