use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use std::collections::HashMap;
use ts_rs::TS;
use url::Url;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicProfileContent {
    pub description: Option<String>,
    pub image: Option<String>,
    pub id: String,
    pub username: String,
    #[ts(type = "string")]
    pub website: Option<Url>,
    pub signature: String,
}

impl Content for WitnessedBasicProfileContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1",
        ]))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "WitnessedBasicProfile".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        let t = vec!["WitnessedBasicProfile".to_string()];
        let mut m = Map::new();
        m.insert("type".to_string(), t.into());
        m.insert("username".to_string(), self.username.clone().into());
        m.insert("id".to_string(), self.id.clone().into());

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
        let mut evidence_map = HashMap::new();
        evidence_map.insert(
            "signature".to_string(),
            serde_json::Value::String(self.signature.clone()),
        );
        let e = Evidence {
            id: None,
            type_: vec!["WitnessedSelfIssuedEvidence".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(e)))
    }
}
