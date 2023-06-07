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
pub struct DappPreferencesAttestationStatement {
    pub subject: Subjects,
    pub dark_mode: bool,
}

impl Attestation for DappPreferencesAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        Ok((
            AttestationTypes::DappPreferencesAttestation,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "dark_mode": self.dark_mode,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
