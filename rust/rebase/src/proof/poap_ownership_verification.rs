use crate::{
    content::poap_ownership_verification::PoapOwnershipVerification as Ctnt,
    statement::poap_ownership_verification::PoapOwnershipVerification as Stmt,
    types::{
        defs::{Proof, Statement},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "proof")]
pub struct PoapOwnershipVerification {
    pub signature: String,
    pub statement: Stmt,
}

impl Statement for PoapOwnershipVerification {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for PoapOwnershipVerification {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            event_id: format!("{}", self.statement.event_id.clone()),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
