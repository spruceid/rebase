use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct DnsVerification {
    pub domain: String,
    pub prefix: String,
    pub subject: Subjects,
}

impl Statement for DnsVerification {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to {}",
            self.domain,
            self.subject.display_id()?
        ))
    }
}
