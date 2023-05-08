use crate::{
    content::witnessed_self_issued::witnessed_progress_book_link::WitnessedProgressBookLinkContent,
    statement::witnessed_self_issued::witnessed_progress_book_link::WitnessedProgressBookLinkStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedProgressBookLinkProof {
    pub statement: WitnessedProgressBookLinkStatement,
    pub signature: String,
}

impl Statement for WitnessedProgressBookLinkProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedProgressBookLinkContent> for WitnessedProgressBookLinkProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedProgressBookLinkContent, ProofError> {
        Ok(WitnessedProgressBookLinkContent {
            id: self.statement.subject.did()?,
            link: self.statement.link.clone(),
            signature: self.signature.clone(),
            progress: self.statement.progress,
        })
    }
}
