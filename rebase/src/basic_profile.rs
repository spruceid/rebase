use crate::schema_type::SchemaType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{
    one_or_many::OneOrMany,
    vc::{Evidence, LinkedDataProofOptions}
};

// TODO: Move to own dir, maybe w/ schema?
#[derive(Deserialize, Serialize)]
pub struct BasicProfile {
    pub alias: String,
    pub description: String,
    pub website: String,
    pub logo: String,
}

impl SchemaType for BasicProfile {
    fn context(&self) -> Result<String, String> {
        Ok(serde_json::from_value(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
              "alias": "https://schema.org/name",
              "description": "https://schema.org/description",
              "website": "https://schema.org/url",
              "logo": "https://schema.org/logo",
              "BasicProfile": "https://tzprofiles.com/BasicProfile",
          },
        ]))
        .map_err(|e| format!("{}", e))?)
    }

    fn vc_types(&self) -> Result<Vec<String>, String> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicProfile".to_string(),
        ])
    }

    fn credential_subject(&self, subject_did: &str) -> Result<String, String> {
        Ok(serde_json::from_value(json!({
            "id": subject_did.to_string(),
            "alias": self.alias,
            "description": self.description,
            "logo": self.logo,
            "website": self.website,
        }))
        .map_err(|e| format!("{}", e))?)
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, String> {
        Ok(None)
    }

    fn proof(&self) -> Result<Option<LinkedDataProofOptions>, String> {
        // TODO: Impl?
        Ok(None)
    }
}
