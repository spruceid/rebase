use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DelegatedBasicProfileAttestationContent {
    pub description: Option<String>,
    pub image: Option<String>,
    pub id: String,
    pub username: String,
    pub website: Option<Url>,
    pub delegate: String,
}

impl Content for DelegatedBasicProfileAttestationContent {
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
            "DelegatedBasicProfileAttestation".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        let t = vec!["DelegatedBasicProfileAttestation".to_string()];
        let mut m = Map::new();
        m.insert("type".to_string(), t.into());
        m.insert("username".to_string(), self.username.clone().into());
        m.insert("id".to_string(), self.id.clone().into());
        m.insert("delegate".to_string(), self.delegate.clone().into());

        if let Some(x) = self.description.clone() {
            m.insert("description".to_string(), x.into());
        };

        if let Some(x) = self.image.clone() {
            m.insert("image".to_string(), x.into());
        };

        if let Some(x) = self.website.clone() {
            m.insert("website".to_string(), x.to_string().into());
        };

        Ok(m.into())
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
