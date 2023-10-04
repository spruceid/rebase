use crate::{
    content::attestation::progress_book_link_attestation::ProgressBookLinkAttestationContent,
    statement::attestation::progress_book_link_attestation::ProgressBookLinkAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ProgressBookLinkAttestationProof {
    pub statement: ProgressBookLinkAttestationStatement,
    pub signature: String,
}

impl Statement for ProgressBookLinkAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<ProgressBookLinkAttestationContent> for ProgressBookLinkAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<ProgressBookLinkAttestationContent, ProofError> {
        Ok(ProgressBookLinkAttestationContent {
            id: self.statement.subject.did()?,
            link: self.statement.link.clone(),
            signature: self.signature.clone(),
            progress: self.statement.progress,
        })
    }
}
