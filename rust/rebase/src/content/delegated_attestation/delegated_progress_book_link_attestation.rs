use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DelegatedProgressBookLinkAttestationContent {
    pub id: String,
    pub link: Url,
    pub progress: i64,
    pub delegate: String,
}

impl Content for DelegatedProgressBookLinkAttestationContent {
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
            "DelegatedProgressBookLinkAttestation".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.id,
            "link": self.link,
            "progress": self.progress,
            "delegate": self.delegate,
            "type": ["DelegatedProgressBookLinkAttestation"],
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
