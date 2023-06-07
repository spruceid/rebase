use crate::types::{
    defs::{Content, Subject},
    enums::subject::Subjects,
    error::ContentError,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct SameControllerAssertionContent {
    pub id1: Subjects,
    pub id2: Subjects,
    pub statement: String,
    pub signature1: String,
    pub signature2: String,
}

impl Content for SameControllerAssertionContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1",
            "https://schema.org/"
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "signature1".to_string(),
            serde_json::Value::String(self.signature1.clone()),
        );

        evidence_map.insert(
            "signature2".to_string(),
            serde_json::Value::String(self.signature2.clone()),
        );

        evidence_map.insert(
            "statement".to_string(),
            serde_json::Value::String(self.statement.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["SameControllerEvidence".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id1": self.id1.did()?,
            "id2": self.id2.did()?,
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(serde_json::from_value(json!([
            "VerifiableCredential",
            "SameControllerAssertion",
        ]))?)
    }
}
