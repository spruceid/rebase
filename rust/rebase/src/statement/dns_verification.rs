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
pub struct DnsVerificationStatement {
    pub domain: String,
    pub prefix: String,
    pub subject: Subjects,
}

impl Statement for DnsVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to {}",
            self.domain,
            self.subject.display_id()?
        ))
    }
}
