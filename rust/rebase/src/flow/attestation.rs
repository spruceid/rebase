use crate::{
    content::attestation::content::AttestationContent,
    proof::attestation::proof::AttestationProof,
    statement::attestation::statement::AttestationStatement,
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

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AttestationFlow {}

#[async_trait(?Send)]
impl Flow<AttestationContent, AttestationStatement, AttestationProof> for AttestationFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        // NOTE: These instructions are for all witnessed flows.
        Ok(Instructions {
            statement: "Fill out the presented form to create content in the form of a credential."
                .to_string(),
            statement_schema: schema_for!(AttestationStatement),
            signature: "Sign a plain-text version of the content created in the previous step."
                .to_string(),
            witness: "Present the signature and the content object to the witness to have it transformed into a credential.".to_string(),
            witness_schema: schema_for!(AttestationProof)
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &AttestationStatement,
        _issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &AttestationProof,
        _issuer: &I,
    ) -> Result<AttestationContent, FlowError> {
        let stmt = proof.generate_statement()?;
        proof
            .subject()
            .valid_signature(&stmt, &proof.signature())
            .await?;
        Ok(proof.to_content(&stmt, &proof.signature())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        proof::attestation::basic_post_attestation::BasicPostAttestationProof,
        statement::attestation::basic_post_attestation::BasicPostAttestationStatement,
        test_util::util::test_did_keypair,
        types::defs::{Issuer, Statement},
    };

    #[tokio::test]
    async fn mock_attestation_flow() {
        let (subj1, iss1) = test_did_keypair().await.unwrap();
        let (_, iss2) = test_did_keypair().await.unwrap();
        let flow = AttestationFlow {};

        let statement = BasicPostAttestationStatement {
            subject: subj1,
            reply_to: None,
            body: "World".to_string(),
            title: "Hello".to_string(),
        };

        let s = statement.generate_statement().unwrap();

        let signature = iss1.sign(&s).await.unwrap();

        let proof = BasicPostAttestationProof {
            statement: statement.clone(),
            signature: signature.clone(),
        };

        flow.validate_proof(
            &AttestationProof::BasicPostAttestation(proof.clone()),
            &iss2,
        )
        .await
        .unwrap();

        let bad_sig = iss2.sign(&s).await.unwrap();
        let bad_proof = BasicPostAttestationProof {
            statement: statement.clone(),
            signature: bad_sig,
        };

        if flow
            .validate_proof(&AttestationProof::BasicPostAttestation(bad_proof), &iss2)
            .await
            .is_ok()
        {
            panic!("Flow approved bad signature")
        }
    }
}
