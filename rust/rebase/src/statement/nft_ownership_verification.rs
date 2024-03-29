use crate::types::{
    defs::{AlchemyNetworks, Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use chrono::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NftOwnershipVerificationStatement {
    pub contract_address: String,
    pub subject: Subjects,
    pub network: AlchemyNetworks,
    pub issued_at: String,
}

impl Statement for NftOwnershipVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        DateTime::parse_from_rfc3339(&self.issued_at)
            .map_err(|e| StatementError::Statement(format!("failed to parse issued_at: {}", e)))?;

        Ok(format!(
            "The {} {} owns an asset from the contract {} on the network {} at time of {}",
            self.subject.statement_title()?,
            self.subject.display_id()?,
            self.contract_address,
            self.network.to_string(),
            self.issued_at,
        ))
    }
}
