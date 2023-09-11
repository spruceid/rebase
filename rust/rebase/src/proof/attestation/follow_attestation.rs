use crate::{
    content::attestation::follow_attestation::FollowAttestationContent,
    statement::attestation::follow_attestation::FollowAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FollowAttestationProof {
    pub statement: FollowAttestationStatement,
    pub signature: String,
}

impl Statement for FollowAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<FollowAttestationContent> for FollowAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<FollowAttestationContent, crate::types::error::ProofError> {
        Ok(FollowAttestationContent {
            id: self.statement.subject.did()?,
            target: self.statement.target.clone(),
            signature: self.signature.clone(),
        })
    }
}
