use crate::types::{
    enums::subject::Subjects,
    error::StatementError,
    types::{Statement, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct Dns {
    pub domain: String,
    pub prefix: String,
    // TODO: CHANGE WHEN READY FOR BREAKING CHANGES:
    #[serde(rename = "key_type")]
    pub subject: Subjects,
}

impl Statement for Dns {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to {}",
            self.domain,
            self.subject.display_id()?
        ))
    }
}
