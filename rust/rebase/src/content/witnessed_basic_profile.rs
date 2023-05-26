use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;
use url::Url;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicProfileContent {
    pub description: String,
    pub image: String,
    pub id: String,
    pub username: String,
    #[ts(type = "string")]
    pub website: Url,
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
        Ok(json!({
            "description": self.description,
            "id": self.id,
            "image": self.image,
            "type": ["WitnessedBasicProfile"],
            "username": self.username,
            "website": self.website.to_string(),
            "signature": self.signature
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
