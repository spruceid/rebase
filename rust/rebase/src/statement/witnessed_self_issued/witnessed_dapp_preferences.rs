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
pub struct WitnessedDappPreferencesStatement {
    pub subject: Subjects,
    pub dark_mode: bool,
}

impl WitnesssedSelfIssued for WitnessedDappPreferencesStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        Ok((
            WitnessedSelfIssuedTypes::WitnessedDappPreferences,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "dark_mode": self.dark_mode,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
