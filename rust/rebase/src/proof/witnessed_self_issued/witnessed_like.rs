use crate::{
    content::witnessed_self_issued::witnessed_like::WitnessedLikeContent,
    statement::witnessed_self_issued::witnessed_like::WitnessedLikeStatement,
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
pub struct WitnessedLikeProof {
    pub statement: WitnessedLikeStatement,
    pub signature: String,
}

impl Statement for WitnessedLikeProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedLikeContent> for WitnessedLikeProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedLikeContent, crate::types::error::ProofError> {
        Ok(WitnessedLikeContent {
            id: self.statement.subject.did()?,
            target: self.statement.target.clone(),
            signature: self.signature.clone(),
        })
    }
}
