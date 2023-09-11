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

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ProgressBookLinkAttestationStatement {
    pub subject: Subjects,
    pub link: Url,
    pub progress: i64,
}

impl Attestation for ProgressBookLinkAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        Ok((
            AttestationTypes::ProgressBookLinkAttestation,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "link": self.link,
                "progress": self.progress,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
