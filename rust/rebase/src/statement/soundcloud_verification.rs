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
pub struct SoundCloudVerificationStatement {
    pub permalink: String,
    pub subject: Subjects,
}

impl Statement for SoundCloudVerificationStatement {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this SoundCloud profile https://soundcloud.com/{} is linked to the {} {}",
            self.permalink,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
