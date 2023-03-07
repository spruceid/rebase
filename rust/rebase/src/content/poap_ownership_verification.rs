use crate::types::{
    defs::{Content, Subject},
    enums::subject::Subjects,
    error::ContentError,
};
use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
pub struct PoapOwnershipVerification {
    pub event_id: String,
    pub subject: Subjects,
    pub statement: String,
    pub signature: String,
}

impl Content for PoapOwnershipVerification {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1"
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "event_id".to_string(),
            serde_json::Value::String(self.event_id.clone()),
        );

        evidence_map.insert(
            "statement".to_string(),
            serde_json::Value::String(self.statement.clone()),
        );

        evidence_map.insert(
            "signature".to_string(),
            serde_json::Value::String(self.signature.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["PoapOwnershipMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject.did()?,
            "event_id": self.event_id.clone(),
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "PoapOwnershipVerification".to_owned(),
        ])
    }
}
