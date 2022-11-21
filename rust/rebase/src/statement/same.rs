use crate::types::{
    enums::subject::Subjects,
    error::StatementError,
    types::{Statement, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct Same {
    pub id1: Subjects,
    pub id2: Subjects,
}

impl Statement for Same {
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
