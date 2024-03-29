use crate::types::{
    defs::{Content, Subject},
    enums::subject::Subjects,
    error::ContentError,
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SameControllerAssertionContent {
    pub id1: Subjects,
    pub id2: Subjects,
    pub statement: String,
    pub signature1: String,
    pub signature2: String,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
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
