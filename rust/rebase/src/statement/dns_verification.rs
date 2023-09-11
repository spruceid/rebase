use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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
