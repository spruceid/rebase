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
pub struct EmailVerificationStatement {
    pub email: String,
    pub subject: Subjects,
}

impl Statement for EmailVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "{} is linked to the {} {}",
            self.email,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
