use crate::{
    content::attestation::like_attestation::LikeAttestationContent,
    statement::attestation::like_attestation::LikeAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LikeAttestationProof {
    pub statement: LikeAttestationStatement,
    pub signature: String,
}

impl Statement for LikeAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<LikeAttestationContent> for LikeAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<LikeAttestationContent, crate::types::error::ProofError> {
        Ok(LikeAttestationContent {
            id: self.statement.subject.did()?,
            target: self.statement.target.clone(),
            signature: self.signature.clone(),
        })
    }
}
