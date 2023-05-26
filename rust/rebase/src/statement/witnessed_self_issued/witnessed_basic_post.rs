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

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicPostStatement {
    pub subject: Subjects,
    pub body: String,
    pub title: String,
    pub reply_to: Option<String>,
}

impl WitnesssedSelfIssued for WitnessedBasicPostStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        let mut m = Map::new();
        m.insert(
            "id".to_string(),
            self.subject
                .did()
                .map_err(|e| StatementError::Statement(e.to_string()))?
                .into(),
        );
        m.insert("body".to_string(), self.body.clone().into());
        m.insert("title".to_string(), self.title.clone().into());

        if let Some(x) = self.reply_to.clone() {
            m.insert("reply_to".to_string(), x.into());
        }

        Ok((WitnessedSelfIssuedTypes::WitnessedBasicPost, m))
    }
}
