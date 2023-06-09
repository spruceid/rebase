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

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct BasicImageAttestationStatement {
    pub subject: Subjects,
    pub src: String,
}

impl Attestation for BasicImageAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        Ok((
            AttestationTypes::BasicImageAttestation,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "src": self.src,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
