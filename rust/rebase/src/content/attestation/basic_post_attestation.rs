use crate::types::{defs::Content, enums::attestation::AttestationFormat, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;

#[derive(Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct BasicPostAttestationContent {
    pub attestation_format: AttestationFormat,
    pub id: String,
    pub body: String,
    pub title: String,
    pub reply_to: Option<String>,
}

impl BasicPostAttestationContent {
    fn get_type(&self) -> String {
        match self.attestation_format {
            AttestationFormat::Attestation => "BasicPostAttestation".to_string(),
            AttestationFormat::DelegatedAttestation => "BasicPostDelegatedAttestation".to_string(),
        }
    }
}

impl Content for BasicPostAttestationContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1",
            "https://schema.org/"
        ]))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec!["VerifiableCredential".to_string(), self.get_type()])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        let t = vec![self.get_type()];
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
        Ok(None)
    }
}
