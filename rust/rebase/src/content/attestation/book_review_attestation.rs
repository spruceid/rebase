use crate::types::{defs::Content, enums::attestation::AttestationFormat, error::ContentError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;
use url::Url;

#[derive(Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub struct BookReviewAttestationContent {
    pub attestation_format: AttestationFormat,
    pub id: String,
    #[ts(type = "string")]
    pub link: Url,
    // Not using u64 because of issues with the JS side JSON schema lib.
    pub rating: i64,
    pub review: String,
    pub signature: String,
    pub title: String,
}

impl BookReviewAttestationContent {
    fn get_type(&self) -> String {
        match self.attestation_format {
            AttestationFormat::Attestation => "BookReviewAttestation".to_string(),
            AttestationFormat::DelegatedAttestation => "BookReviewDelegatedAttestation".to_string(),
        }
    }
}

impl Content for BookReviewAttestationContent {
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
            "link": self.link,
            "rating": self.rating,
            "review": self.review,
            "title": self.title,
            "type": [self.get_type()],
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
