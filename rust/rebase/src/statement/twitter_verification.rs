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
pub struct TwitterVerificationStatement {
    pub handle: String,
    pub subject: Subjects,
}

impl Statement for TwitterVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this twitter handle @{} is linked to the {} {}",
            self.handle,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
