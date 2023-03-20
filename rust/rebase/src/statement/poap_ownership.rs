use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use chrono::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct PoapOwnership {
    // NOTE: This should ideally be a u64
    // i64 is being used because it comes out
    // correctly in the JSON Schema
    // The Rust JSON Schema lib adds a marker
    // for the u64 that the JS lib cannot understand
    // There is likely a clean way to use u64, but
    // the only consequence of a negative event id
    // is not finding anything on look up.
    pub event_id: i64,
    pub issued_at: String,
    pub subject: Subjects,
}

impl Statement for PoapOwnership {
    fn generate_statement(&self) -> Result<String, StatementError> {
        DateTime::parse_from_rfc3339(&self.issued_at)
            .map_err(|e| StatementError::Statement(format!("failed to parse issued_at: {}", e)))?;

        Ok(format!(
            "The {} {} has a POAP for event id {} at time of {}",
            self.subject.statement_title()?,
            self.subject.display_id()?,
            self.event_id,
            self.issued_at,
        ))
    }
}
