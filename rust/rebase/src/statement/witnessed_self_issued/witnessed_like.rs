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
use url::Url;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedLikeStatement {
    pub subject: Subjects,
    #[ts(type = "string")]
    pub target: Url,
}

impl WitnesssedSelfIssued for WitnessedLikeStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        Ok((
            WitnessedSelfIssuedTypes::WitnessedLike,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "target": self.target,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
