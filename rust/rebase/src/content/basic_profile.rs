use crate::types::{error::ContentError, types::Content};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Deserialize, Serialize)]
pub struct BasicProfile {
    pub alias: String,
    pub description: String,
    // TODO: Type as URL?
    pub website: String,
    pub logo: String,
    pub subject_id: String,
}

impl Content for BasicProfile {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
              "BasicProfile": {
                    "@id": "https://example.com/BasicProfile",
                    "@context": {
                        "alias": "https://schema.org/name",
                        "description": "https://schema.org/description",
                        "website": "https://schema.org/url",
                        "logo": "https://schema.org/logo",
                    }
                }
            },
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
            "logo": self.logo,
            "website": self.website,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
