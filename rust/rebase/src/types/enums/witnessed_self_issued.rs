use crate::types::{defs::Statement, error::StatementError};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use ts_rs::TS;

pub trait WitnesssedSelfIssued {
    fn to_content(&self) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError>;
}

impl<T: WitnesssedSelfIssued> Statement for T {
    fn generate_statement(&self) -> Result<String, StatementError> {
        let (t, content) = self.to_content()?;
        t.validate(&content)?;
        let mut v: Vec<String> = Vec::new();
        for k in content.keys() {
            match content.get(k) {
                None => {
                    return Err(StatementError::Statement(format!(
                        "Could not find entry {} in content",
                        k
                    )))
                }
                Some(val) => {
                    let s = format!("{}:{}", k, val);
                    v.push(s);
                }
            }
        }
        v.sort();

        Ok(v.join("\n"))
    }
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub enum WitnessedSelfIssuedTypes {
    WitnessedBasicPost,
    WitnessedBasicProfile,
}

impl WitnessedSelfIssuedTypes {
    // Returns a map of keys in the content, along with a boolean for if they are optional.
    pub fn to_key_map(&self) -> HashMap<String, bool> {
        match &self {
            WitnessedSelfIssuedTypes::WitnessedBasicPost => HashMap::from([
                ("body".to_string(), true),
                ("id".to_string(), true),
                ("title".to_string(), false),
                ("reply_to".to_string(), false),
            ]),
            WitnessedSelfIssuedTypes::WitnessedBasicProfile => HashMap::from([
                ("description".to_string(), true),
                ("id".to_string(), true),
                ("image".to_string(), true),
                ("username".to_string(), true),
                ("website".to_string(), true),
            ]),
        }
    }

    pub fn validate(&self, content: &Map<String, Value>) -> Result<(), StatementError> {
        let h = self.to_key_map();
        for expected_key in h.keys() {
            match h.get(expected_key) {
                None => {
                    return Err(StatementError::Statement("Should be impossible, could not find entry in HashMap even though the key was found by iterating over the HashMap, good luck with this error".to_string()))
                }
                Some(b) => {
                    if *b && content.get(expected_key).is_none() {
                        return Err(StatementError::Statement(format!(
                            "Could not find required entry {}",
                            expected_key
                        )));
                    }
                }
            };
        }

        for expected_key in content.keys() {
            if h.get(expected_key).is_none() {
                return Err(StatementError::Statement(format!(
                    "Found unknown key in content: {}",
                    expected_key
                )));
            };
        }

        Ok(())
    }
}
