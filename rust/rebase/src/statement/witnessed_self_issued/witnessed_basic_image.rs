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
pub struct WitnessedBasicImageStatement {
    pub subject: Subjects,
    pub src: String,
}

impl WitnesssedSelfIssued for WitnessedBasicImageStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        Ok((
            WitnessedSelfIssuedTypes::WitnessedBasicImage,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "src": self.src,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
