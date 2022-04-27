use crate::schema::schema_type::{SchemaError, SchemaType};
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
}

impl SchemaType for BasicProfile {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
              "alias": "https://schema.org/name",
              "description": "https://schema.org/description",
              "website": "https://schema.org/url",
              "logo": "https://schema.org/logo",
              // TODO: Establish new place for this URL to point.
              "BasicProfile": "https://tzprofiles.com/BasicProfile",
          },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicProfile".to_string(),
        ])
    }

    fn subject(&self, subject_did: &str) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "id": subject_did.to_string(),
            "alias": self.alias,
            "description": self.description,
            "logo": self.logo,
            "website": self.website,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Ok(None)
    }
}
