use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use chrono::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct EmailVerificationStatement {
    pub email: String,
    pub subject: Subjects,
}

impl Statement for EmailVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to the {} {}",
            self.email,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
