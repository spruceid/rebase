use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use std::collections::HashMap;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BasicTagAttestationContent {
    pub id: String,
    pub post: String,
    pub users: Vec<String>,
    pub signature: String,
}

impl Content for BasicTagAttestationContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1",
            "https://schema.org/"
        ]))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicTagAttestation".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.id,
            "post": self.post,
            "users": self.users,
            "type": ["BasicTagAttestation"],
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = HashMap::new();
        evidence_map.insert(
            "signature".to_string(),
            serde_json::Value::String(self.signature.clone()),
        );
        let e = Evidence {
            id: None,
            type_: vec!["AttestationEvidence".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(e)))
    }
}
