use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct BasicProfileContent {
    pub alias: String,
    pub description: String,
    // TODO: Type as URL?
    pub website: String,
    pub image: String,
    pub subject_id: String,
}

impl Content for BasicProfileContent {
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
            "BasicProfile".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject_id,
            "type": ["BasicProfile"],
            "alias": self.alias,
            "description": self.description,
            "image": self.image,
            "website": self.website,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
