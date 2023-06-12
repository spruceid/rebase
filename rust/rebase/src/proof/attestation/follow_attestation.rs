use crate::{
    content::attestation::follow_attestation::FollowAttestationContent,
    statement::attestation::follow_attestation::FollowAttestationStatement,
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
    ) -> Result<FollowAttestationContent, ProofError> {
        Ok(FollowAttestationContent {
            attestation_format: AttestationFormat::Attestation,
            id: self.statement.subject.did()?,
            target: self.statement.target.clone(),
            signature: self.signature.clone(),
        })
    }
}
