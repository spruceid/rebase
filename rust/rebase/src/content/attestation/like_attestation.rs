use crate::types::{defs::Content, enums::attestation::AttestationFormat, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;
use url::Url;

#[derive(Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct LikeAttestationContent {
    pub attestation_format: AttestationFormat,
    pub id: String,
    #[ts(type = "string")]
    pub target: Url,
    pub signature: String,
}

impl LikeAttestationContent {
    fn get_type(&self) -> String {
        match self.attestation_format {
            AttestationFormat::Attestation => "LikeAttestation".to_string(),
            AttestationFormat::DelegatedAttestation => "LikeDelegatedAttestation".to_string(),
        }
    }
}

impl Content for LikeAttestationContent {
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
        Ok(json!({
            "id": self.id,
            "target": self.target,
            "type": [self.get_type()],
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
