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
