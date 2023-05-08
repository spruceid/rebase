use crate::{
    content::witnessed_self_issued::witnessed_follow::WitnessedFollowContent,
    statement::witnessed_self_issued::witnessed_follow::WitnessedFollowStatement,
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
pub struct WitnessedFollowProof {
    pub statement: WitnessedFollowStatement,
    pub signature: String,
}

impl Statement for WitnessedFollowProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedFollowContent> for WitnessedFollowProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedFollowContent, crate::types::error::ProofError> {
        Ok(WitnessedFollowContent {
            id: self.statement.subject.did()?,
            target: self.statement.target.clone(),
            signature: self.signature.clone(),
        })
    }
}
