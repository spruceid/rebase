use crate::types::{
    defs::{Statement, Subject},
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
pub struct WitnessedBasicProfileStatement {
    pub description: String,
    pub image: String,
    pub subject: Subjects,
    pub username: String,
    #[ts(type = "string")]
    pub website: Url,
}

impl WitnesssedSelfIssued for WitnessedBasicProfileStatement {
    fn to_content(&self) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        Ok((
            WitnessedSelfIssuedTypes::WitnessedBasicProfile,
            serde_json::from_value(json!({
                "description": self.description,
                "id": self.subject.did()?,
                "image": self.image,
                "username": self.username,
                "website": self.website
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
