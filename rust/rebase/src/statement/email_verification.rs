use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct EmailVerification {
    pub email: String,
    pub subject: Subjects,
}

impl Statement for EmailVerification {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to the {} {}",
            self.email,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
