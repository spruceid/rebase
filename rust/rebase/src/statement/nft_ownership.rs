use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: Change this to an enum of possible chains / details.
// Will match an Alchemy specific instance of NftOwnership
// As SendGrid is to Email.
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct NftOwnership {
    pub contract_address: String,
    pub subject: Subjects,
    pub network: String,
    pub issued_at: String,
}

impl Statement for NftOwnership {
    fn generate_statement(&self) -> Result<String, StatementError> {
        // TODO: Parse issued_at for valid date.
        Ok(format!(
            "The {} {} owns an asset from the contract {} on the network {} at time of {}",
            self.subject.statement_title()?,
            self.subject.display_id()?,
            self.contract_address,
            self.network,
            self.issued_at,
        ))
    }
}
