use crate::types::{defs::Content, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;

#[derive(Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct DelegatedBasicTagAttestationContent {
    pub id: String,
    pub post: String,
    pub users: Vec<String>,
    pub delegate: String,
}

impl Content for DelegatedBasicTagAttestationContent {
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
            "DelegatedBasicTagAttestation".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.id,
            "post": self.post,
            "users": self.users,
            "delegate": self.delegate,
            "type": ["DelegatedBasicTagAttestation"],
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
