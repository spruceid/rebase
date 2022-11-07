use crate::types::{
    enums::subject::Subjects,
    error::StatementError,
    types::{Statement, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct TwoKey {
    // TODO: RENAME AFTER ISSUE RESOLVES!
    #[serde(rename = "key_1")]
    pub subject1: Subjects,
    #[serde(rename = "key_2")]
    pub subject2: Subjects,
}

impl Statement for TwoKey {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that {} {} is linked to {} {}",
            self.subject1.statement_title()?,
            self.subject1.display_id()?,
            self.subject2.statement_title()?,
            self.subject2.display_id()?
        ))
    }
}
