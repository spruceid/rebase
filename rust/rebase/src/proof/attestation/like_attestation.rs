use crate::{
    content::attestation::like_attestation::LikeAttestationContent,
    statement::attestation::like_attestation::LikeAttestationStatement,
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
    ) -> Result<LikeAttestationContent, ProofError> {
        Ok(LikeAttestationContent {
            attestation_format: AttestationFormat::Attestation,
            id: self.statement.subject.did()?,
            target: self.statement.target.clone(),
        })
    }
}
