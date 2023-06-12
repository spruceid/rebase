use crate::{
    content::attestation::basic_tag_attestation::BasicTagAttestationContent,
    statement::attestation::basic_tag_attestation::BasicTagAttestationStatement,
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
    ) -> Result<BasicTagAttestationContent, ProofError> {
        Ok(BasicTagAttestationContent {
            attestation_format: AttestationFormat::Attestation,
            id: self.statement.subject.did()?,
            users: self.statement.users.clone(),
            post: self.statement.post.clone(),
        })
    }
}
