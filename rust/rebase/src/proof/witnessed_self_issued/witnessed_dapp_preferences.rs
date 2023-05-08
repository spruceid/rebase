use crate::{
    content::witnessed_self_issued::witnessed_dapp_preferences::WitnessedDappPreferencesContent,
    statement::witnessed_self_issued::witnessed_dapp_preferences::WitnessedDappPreferencesStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedDappPreferencesProof {
    pub statement: WitnessedDappPreferencesStatement,
    pub signature: String,
}

impl Statement for WitnessedDappPreferencesProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedDappPreferencesContent> for WitnessedDappPreferencesProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedDappPreferencesContent, crate::types::error::ProofError> {
        Ok(WitnessedDappPreferencesContent {
            id: self.statement.subject.did()?,
            dark_mode: self.statement.dark_mode,
            signature: self.signature.clone(),
        })
    }
}
