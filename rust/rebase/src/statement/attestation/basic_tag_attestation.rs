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
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BasicTagAttestationStatement {
    pub subject: Subjects,
    pub post: String,
    pub users: Vec<String>,
}

impl Attestation for BasicTagAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        Ok((
            AttestationTypes::BasicTagAttestation,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "post": self.post,
                "users": self.users,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
