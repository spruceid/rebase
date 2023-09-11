use crate::{
    content::attestation::basic_post_attestation::BasicPostAttestationContent,
    statement::attestation::basic_post_attestation::BasicPostAttestationStatement,
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
pub struct BasicPostAttestationProof {
    pub statement: BasicPostAttestationStatement,
    pub signature: String,
}

impl Statement for BasicPostAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<BasicPostAttestationContent> for BasicPostAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<BasicPostAttestationContent, crate::types::error::ProofError> {
        Ok(BasicPostAttestationContent {
            id: self.statement.subject.did()?,
            title: self.statement.title.clone(),
            body: self.statement.body.clone(),
            reply_to: self.statement.reply_to.clone(),
            signature: self.signature.clone(),
        })
    }
}
