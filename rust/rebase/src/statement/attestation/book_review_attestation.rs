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
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BookReviewAttestationStatement {
    pub subject: Subjects,
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
