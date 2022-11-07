use crate::types::{
    enums::subject::Subjects,
    error::StatementError,
    types::{Statement, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct Email {
    pub email: String,
    // TODO: CHANGE WHEN READY FOR BREAKING CHANGES:
    #[serde(rename = "key_type")]
    pub subject: Subjects,
}

impl Statement for Email {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to the {} {}",
            self.email,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
