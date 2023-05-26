use crate::{
    content::witnessed_self_issued::witnessed_basic_tag::WitnessedBasicTagContent,
    statement::witnessed_self_issued::witnessed_basic_tag::WitnessedBasicTagStatement,
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
pub struct WitnessedBasicTagProof {
    pub statement: WitnessedBasicTagStatement,
    pub signature: String,
}

impl Statement for WitnessedBasicTagProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedBasicTagContent> for WitnessedBasicTagProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedBasicTagContent, crate::types::error::ProofError> {
        Ok(WitnessedBasicTagContent {
            id: self.statement.subject.did()?,
            users: self.statement.users.clone(),
            post: self.statement.post.clone(),
            signature: self.signature.clone(),
        })
    }
}
