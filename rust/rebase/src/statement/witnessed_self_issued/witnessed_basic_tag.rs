use crate::types::{
    defs::Subject,
    enums::{
        subject::Subjects,
        witnessed_self_issued::{WitnessedSelfIssuedTypes, WitnesssedSelfIssued},
    },
    error::StatementError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicTagStatement {
    pub subject: Subjects,
    pub post: String,
    pub users: Vec<String>,
}

impl WitnesssedSelfIssued for WitnessedBasicTagStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        Ok((
            WitnessedSelfIssuedTypes::WitnessedBasicTag,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "post": self.post,
                "users": self.users,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
