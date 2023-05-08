use crate::{
    content::witnessed_self_issued::witnessed_basic_post::WitnessedBasicPostContent,
    statement::witnessed_self_issued::witnessed_basic_post::WitnessedBasicPostStatement,
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
pub struct WitnessedBasicPostProof {
    pub statement: WitnessedBasicPostStatement,
    pub signature: String,
}

impl Statement for WitnessedBasicPostProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedBasicPostContent> for WitnessedBasicPostProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedBasicPostContent, crate::types::error::ProofError> {
        Ok(WitnessedBasicPostContent {
            id: self.statement.subject.did()?,
            title: self.statement.title.clone(),
            body: self.statement.body.clone(),
            reply_to: self.statement.reply_to.clone(),
            signature: self.signature.clone(),
        })
    }
}
