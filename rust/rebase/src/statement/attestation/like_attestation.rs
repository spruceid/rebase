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
pub struct LikeAttestationStatement {
    pub subject: Subjects,
    #[ts(type = "string")]
    pub target: Url,
}

impl Attestation for LikeAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        Ok((
            AttestationTypes::LikeAttestation,
            serde_json::from_value(json!({
                "id": self.subject.did()?,
                "target": self.target,
            }))
            .map_err(|e| StatementError::Statement(e.to_string()))?,
        ))
    }
}
