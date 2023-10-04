use crate::{
    content::attestation::dapp_preferences_attestation::DappPreferencesAttestationContent,
    statement::attestation::dapp_preferences_attestation::DappPreferencesAttestationStatement,
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
    ) -> Result<DappPreferencesAttestationContent, crate::types::error::ProofError> {
        Ok(DappPreferencesAttestationContent {
            id: self.statement.subject.did()?,
            dark_mode: self.statement.dark_mode,
            signature: self.signature.clone(),
        })
    }
}
