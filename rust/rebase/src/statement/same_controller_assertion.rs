use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SameControllerAssertionStatement {
    pub id1: Subjects,
    pub id2: Subjects,
}

impl Statement for SameControllerAssertionStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that {} {} is linked to {} {}",
            self.id1.statement_title()?,
            self.id1.display_id()?,
            self.id2.statement_title()?,
            self.id2.display_id()?
        ))
    }
}
