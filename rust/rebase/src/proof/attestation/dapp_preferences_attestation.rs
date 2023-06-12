use crate::{
    content::attestation::dapp_preferences_attestation::DappPreferencesAttestationContent,
    statement::attestation::dapp_preferences_attestation::DappPreferencesAttestationStatement,
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
pub struct DappPreferencesAttestationProof {
    pub statement: DappPreferencesAttestationStatement,
    pub signature: String,
}

impl Statement for DappPreferencesAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<DappPreferencesAttestationContent> for DappPreferencesAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<DappPreferencesAttestationContent, ProofError> {
        Ok(DappPreferencesAttestationContent {
            attestation_format: AttestationFormat::Attestation,
            id: self.statement.subject.did()?,
            dark_mode: self.statement.dark_mode,
            signature: self.signature.clone(),
        })
    }
}
