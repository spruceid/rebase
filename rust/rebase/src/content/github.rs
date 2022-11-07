use crate::types::{
    enums::subject::Subjects,
    error::ContentError,
    types::{Content, Subject},
};
use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
pub struct GitHub {
    pub gist_id: String,
    pub handle: String,
    pub subject: Subjects,
    pub statement: String,
    pub signature: String,
}

impl Content for GitHub {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        // TODO: MAKE THESE URLS POINT ELSEWHERE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "GitHubVerification": "https://example.com/GitHubVerification",
                "GitHubVerificationMessage": {
                    "@id": "https://example.com/GitHubVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "gistId": "https://example.com/gistId",
                        // "gistVersion":  "https://example.com/gistVersion",
                        "handle": "https://example.com/handle"
                    }
                }
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "handle".to_string(),
            serde_json::Value::String(self.handle.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        evidence_map.insert(
            "gistId".to_string(),
            serde_json::Value::String(self.gist_id.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["GitHubVerificationMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject.did()?,
            "sameAs": format!("https://github.com/{}", self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "GitHubVerification".to_owned(),
        ])
    }
}
