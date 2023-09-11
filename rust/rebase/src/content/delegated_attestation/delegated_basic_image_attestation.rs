use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DelegatedBasicImageAttestationContent {
    pub id: String,
    pub src: String,
    pub delegate: String,
}

impl Content for DelegatedBasicImageAttestationContent {
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
            "DelegatedBasicImageAttestation".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.id,
            "src": self.src,
            "delegate": self.delegate,
            "type": ["DelegatedBasicImageAttestation"],
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
