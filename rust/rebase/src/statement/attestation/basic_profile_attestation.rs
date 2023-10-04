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
use serde_json::{Map, Value};
use tsify::Tsify;
use url::Url;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct BasicProfileAttestationStatement {
    pub description: Option<String>,
    pub image: Option<String>,
    pub subject: Subjects,
    pub username: String,
    pub website: Option<Url>,
}

impl Attestation for BasicProfileAttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        let mut m = Map::new();
        m.insert("username".to_string(), self.username.clone().into());
        m.insert(
            "id".to_string(),
            self.subject
                .did()
                .map_err(|e| StatementError::Statement(e.to_string()))?
                .into(),
        );

        if let Some(x) = self.description.clone() {
            m.insert("description".to_string(), x.into());
        };

        if let Some(x) = self.image.clone() {
            m.insert("image".to_string(), x.into());
        };

        if let Some(x) = self.website.clone() {
            m.insert("website".to_string(), x.to_string().into());
        };

        Ok((AttestationTypes::BasicProfileAttestation, m))
    }
}
