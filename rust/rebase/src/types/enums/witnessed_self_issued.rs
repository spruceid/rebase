use crate::types::{defs::Statement, error::StatementError};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use ts_rs::TS;

pub trait WitnesssedSelfIssued {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError>;
}

impl<T: WitnesssedSelfIssued> Statement for T {
    fn generate_statement(&self) -> Result<String, StatementError> {
        let (t, content) = self.to_statement()?;
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
        // NOTE: If supporting languages beyond english, add checks for language code
        // and generate the statement header here!
        v.insert(
            0,
            "Sign a copy of your data to turn it into a Verifiable Credential:\n".to_string(),
        );

        Ok(v.join("\n"))
    }
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub enum WitnessedSelfIssuedTypes {
    WitnessedBasicImage,
    WitnessedBasicPost,
    WitnessedBasicProfile,
    WitnessedBasicTag,
    WitnessedBookReview,
    WitnessedDappPreferences,
    WitnessedFollow,
    WitnessedLike,
    WitnessedProgressBookLink,
}

impl WitnessedSelfIssuedTypes {
    // Returns a map of keys in the content, along with a boolean for if they are optional.
    pub fn to_key_map(&self) -> HashMap<String, bool> {
        match &self {
            WitnessedSelfIssuedTypes::WitnessedBasicImage => {
                HashMap::from([("id".to_string(), true), ("src".to_string(), true)])
            }
            WitnessedSelfIssuedTypes::WitnessedBasicPost => HashMap::from([
                ("body".to_string(), true),
                ("id".to_string(), true),
                // TODO: Determine if title should be optional.
                ("title".to_string(), true),
                ("reply_to".to_string(), false),
            ]),
            WitnessedSelfIssuedTypes::WitnessedBasicProfile => HashMap::from([
                ("description".to_string(), false),
                ("id".to_string(), true),
                ("image".to_string(), false),
                ("username".to_string(), true),
                ("website".to_string(), false),
            ]),
            WitnessedSelfIssuedTypes::WitnessedBasicTag => HashMap::from([
                ("id".to_string(), true),
                ("users".to_string(), true),
                ("post".to_string(), true),
            ]),
            WitnessedSelfIssuedTypes::WitnessedBookReview => HashMap::from([
                ("id".to_string(), true),
                ("link".to_string(), true),
                ("rating".to_string(), true),
                ("review".to_string(), true),
                ("title".to_string(), true),
            ]),
            WitnessedSelfIssuedTypes::WitnessedDappPreferences => {
                HashMap::from([("id".to_string(), true), ("dark_mode".to_string(), true)])
            }
            WitnessedSelfIssuedTypes::WitnessedFollow => {
                HashMap::from([("id".to_string(), true), ("target".to_string(), true)])
            }
            WitnessedSelfIssuedTypes::WitnessedLike => {
                HashMap::from([("id".to_string(), true), ("target".to_string(), true)])
            }
            WitnessedSelfIssuedTypes::WitnessedProgressBookLink => HashMap::from([
                ("id".to_string(), true),
                ("link".to_string(), true),
                ("progress".to_string(), true),
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
