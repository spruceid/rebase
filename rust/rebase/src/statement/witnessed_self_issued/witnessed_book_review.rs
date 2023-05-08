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
pub struct WitnessedBookReviewStatement {
    pub subject: Subjects,
    #[ts(type = "string")]
    pub link: Url,
    pub rating: i64,
    pub review: String,
    pub title: String,
}

impl WitnesssedSelfIssued for WitnessedBookReviewStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        Ok((
            WitnessedSelfIssuedTypes::WitnessedBookReview,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "link": self.link,
                "rating": self.rating,
                "review": self.review,
                "title": self.title
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
