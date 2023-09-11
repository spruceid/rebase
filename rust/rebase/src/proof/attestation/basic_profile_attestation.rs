use crate::{
    content::attestation::basic_profile_attestation::BasicProfileAttestationContent,
    statement::attestation::basic_profile_attestation::BasicProfileAttestationStatement,
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
pub struct BasicProfileAttestationProof {
    pub statement: BasicProfileAttestationStatement,
    pub signature: String,
}

impl Statement for BasicProfileAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<BasicProfileAttestationContent> for BasicProfileAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<BasicProfileAttestationContent, crate::types::error::ProofError> {
        Ok(BasicProfileAttestationContent {
            description: self.statement.description.clone(),
            id: self.statement.subject.did()?,
            image: self.statement.image.clone(),
            username: self.statement.username.clone(),
            website: self.statement.website.clone(),
            signature: self.signature.clone(),
        })
    }
}
