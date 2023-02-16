use crate::types::{
    enums::subject::Subjects,
    error::StatementError,
    types::{Statement, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct PoapOwnership {
    pub event_id: u64,
    pub issued_at: String,
    pub subject: Subjects,
}

impl Statement for PoapOwnership {
    fn generate_statement(&self) -> Result<String, StatementError> {
        // TODO: Parse issued_at for valid date.
        Ok(format!(
            "The {} {} has a POAP for event id {} at time of {}",
            self.subject.statement_title()?,
            self.subject.display_id()?,
            self.event_id,
            self.issued_at,
        ))
    }
}
