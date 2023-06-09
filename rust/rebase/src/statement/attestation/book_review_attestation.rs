use crate::types::{
    defs::Subject,
    enums::{
        attestation::{Attestation, AttestationTypes},
        subject::Subjects,
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
pub struct BookReviewAttestationStatement {
    pub subject: Subjects,
    #[ts(type = "string")]
    pub link: Url,
    pub rating: i64,
    pub review: String,
    pub title: String,
}

impl Attestation for BookReviewAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        Ok((
            AttestationTypes::BookReviewAttestation,
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
