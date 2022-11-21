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
#[serde(rename = "same")]
pub struct Same {
    pub id1: Subjects,
    pub id2: Subjects,
    pub statement: String,
    pub signature1: String,
    pub signature2: String,
}

impl Content for Same {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "id1": "https://example.com/id",
                "id2": "https://example.com/id",
                "SameControllerAssertion": "https://example.com/SameControllerAssertion",
                "SameControllerEvidence": {
                    "@id": "https://example.com/SameControllerEdvidence",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "signature1": "https://example.com/signature_1",
                        "signature2": "https://example.com/signature_2",
                        "statement": "https://example.com/statement",
                    }
                },
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "signature1".to_string(),
            serde_json::Value::String(self.signature1.clone()),
        );

        evidence_map.insert(
            "signature2".to_string(),
            serde_json::Value::String(self.signature2.clone()),
        );

        evidence_map.insert(
            "statement".to_string(),
            serde_json::Value::String(self.statement.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["SameControllerEvidence".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id1": self.id1.did()?,
            "id2": self.id2.did()?,
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(serde_json::from_value(json!([
            "VerifiableCredential",
            "SameControllerAssertion",
        ]))?)
    }
}
