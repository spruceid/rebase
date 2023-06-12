use crate::{
    content::attestation::progress_book_link_attestation::ProgressBookLinkAttestationContent,
    statement::attestation::progress_book_link_attestation::ProgressBookLinkAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        enums::attestation::AttestationFormat,
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
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
            attestation_format: AttestationFormat::Attestation,
            id: self.statement.subject.did()?,
            link: self.statement.link.clone(),
            progress: self.statement.progress,
        })
    }
}
