use crate::types::{error::ContentError, types::Content};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Deserialize, Serialize)]
pub struct BasicPost {
    pub title: String,
    pub body: String,
    pub subject_id: String,
}

impl Content for BasicPost {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        // TODO: MAKE THESE URLS MORE ACCURATE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "BasicPostCredential": "https://example.com/BasicPostCredential",
                "BasicPost": {
                    "@id": "https://schema.org/BasicPost",
                    "@context": {
                        "title": "https://schema.org/name",
                        "body": "https://schema.org/articleBody",
                    }
                }
            },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicPost".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject_id,
            "type": ["BasicPost"],
            "title": self.title,
            "body": self.body,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
