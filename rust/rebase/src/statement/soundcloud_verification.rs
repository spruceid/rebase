use crate::types::{
    defs::{Statement, Subject},
    enums::subject::Subjects,
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct SoundCloudVerification {
    pub permalink: String,
    pub subject: Subjects,
}

impl Statement for SoundCloudVerification {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this SoundCloud profile https://soundcloud.com/{} is linked to the {} {}",
            self.permalink,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
