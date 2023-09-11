use crate::{
    content::attestation::basic_tag_attestation::BasicTagAttestationContent,
    statement::attestation::basic_tag_attestation::BasicTagAttestationStatement,
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
pub struct BasicTagAttestationProof {
    pub statement: BasicTagAttestationStatement,
    pub signature: String,
}

impl Statement for BasicTagAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<BasicTagAttestationContent> for BasicTagAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<BasicTagAttestationContent, crate::types::error::ProofError> {
        Ok(BasicTagAttestationContent {
            id: self.statement.subject.did()?,
            users: self.statement.users.clone(),
            post: self.statement.post.clone(),
            signature: self.signature.clone(),
        })
    }
}
