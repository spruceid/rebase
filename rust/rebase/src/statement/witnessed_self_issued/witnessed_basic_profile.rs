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
use serde_json::{Map, Value};
use ts_rs::TS;
use url::Url;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicProfileStatement {
    pub description: Option<String>,
    pub image: Option<String>,
    pub subject: Subjects,
    pub username: String,
    #[ts(type = "string")]
    pub website: Option<Url>,
}

impl WitnesssedSelfIssued for WitnessedBasicProfileStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        let mut m = Map::new();
        m.insert("username".to_string(), self.username.clone().into());
        m.insert(
            "id".to_string(),
            self.subject
                .did()
                .map_err(|e| StatementError::Statement(e.to_string()))?
                .into(),
        );

        if let Some(x) = self.description.clone() {
            m.insert("description".to_string(), x.into());
        };

        if let Some(x) = self.image.clone() {
            m.insert("image".to_string(), x.into());
        };

        if let Some(x) = self.website.clone() {
            m.insert("website".to_string(), x.to_string().into());
        };

        Ok((WitnessedSelfIssuedTypes::WitnessedBasicProfile, m))
    }
}
