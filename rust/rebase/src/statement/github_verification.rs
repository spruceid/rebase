use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct GitHubVerificationStatement {
    pub handle: String,
    pub subject: Subjects,
}

impl Statement for GitHubVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this GitHub handle {} is linked to the {} {}",
            self.handle,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
