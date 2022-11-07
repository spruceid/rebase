use crate::{
    content::two_key::TwoKey as Ctnt,
    statement::two_key::TwoKey as Stmt,
    types::{
        error::{ProofError, StatementError},
        types::{Proof, Statement},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "proof")]
pub struct TwoKey {
    // TODO: Remove when we're ready for breaking changes
    #[serde(rename = "statement_opts")]
    pub statement: Stmt,
    pub signature_1: String,
    pub signature_2: String,
}

impl Statement for TwoKey {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for TwoKey {
    fn to_content(&self, _statement: &str, _signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            key_1: self.statement.subject1.clone(),
            key_2: self.statement.subject2.clone(),
            statement: self.generate_statement()?,
            signature_1: self.signature_1.clone(),
            signature_2: self.signature_2.clone(),
        })
    }
}
