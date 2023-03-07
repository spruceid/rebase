use crate::{
    content::github_verification::GitHubVerification as Ctnt,
    statement::github_verification::GitHubVerification as Stmt,
    types::{
        defs::{Proof, Statement},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "proof")]
pub struct GitHubVerification {
    pub gist_id: String,
    pub statement: Stmt,
}

impl Statement for GitHubVerification {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for GitHubVerification {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            gist_id: self.gist_id.clone(),
            handle: self.statement.handle.clone(),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
