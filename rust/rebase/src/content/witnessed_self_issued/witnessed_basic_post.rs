use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicPostContent {
    pub id: String,
    pub body: String,
    pub title: String,
    pub reply_to: Option<String>,
    pub signature: String,
}

impl Content for WitnessedBasicPostContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1",
        ]))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "WitnessedBasicPost".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        let t = vec!["WitnessedBasicPost".to_string()];
        let mut m = Map::new();
        m.insert("type".to_string(), t.into());
        m.insert("id".to_string(), self.id.clone().into());
        m.insert("body".to_string(), self.body.clone().into());
        m.insert("title".to_string(), self.title.clone().into());

        if let Some(x) = self.reply_to.clone() {
            m.insert("reply_to".to_string(), x.into());
        }

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
