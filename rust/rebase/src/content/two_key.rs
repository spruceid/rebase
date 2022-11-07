use crate::types::{
    enums::subject::Subjects,
    error::ContentError,
    types::{Content, Subject},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "two_key")]
pub struct TwoKey {
    pub key_1: Subjects,
    pub key_2: Subjects,
    pub statement: String,
    pub signature_1: String,
    pub signature_2: String,
}

impl Content for TwoKey {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "id": "https://example.com/id",
                "sameAs": "http://schema.org/sameAs",
                "SelfSignedControl": "https://example.com/SelfSignedControl",
                "SelfSignedControlVerification": {
                    "@id": "https://example.com/SelfSignedControlVerification",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "signature_1": "https://example.com/signature_1",
                        "signature_2": "https://example.com/signature_2",
                        "statement": "https://example.com/statement",
                    }
                },
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "signature_1".to_string(),
            serde_json::Value::String(self.signature_1.clone()),
        );

        evidence_map.insert(
            "signature_2".to_string(),
            serde_json::Value::String(self.signature_2.clone()),
        );

        evidence_map.insert(
            "statement".to_string(),
            serde_json::Value::String(self.statement.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["SelfSignedControlVerification".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.key_1.did()?,
            "sameAs": self.key_2.did()?,
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(serde_json::from_value(json!([
            "VerifiableCredential",
            "SelfSignedControl",
        ]))?)
    }
}
