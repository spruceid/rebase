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
pub struct Twitter {
    pub handle: String,
    pub subject: Subjects,
    pub tweet_url: String,
    pub statement: String,
    pub signature: String,
}

impl Content for Twitter {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        // TODO: Change where these are pointed
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "TwitterVerification": "https://example.com/TwitterVerification",
                "TwitterVerificationPublicTweet": {
                    "@id": "https://example.com/TwitterVerificationPublicTweet",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "handle": "https://example.com/handle",
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "tweetId": "https://example.com/tweetId"
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
        let url_vec: Vec<&str> = self.tweet_url.split("/").collect();
        if url_vec.len() < 1 {
            return Err(ContentError::Invalid("could not find tweet id".to_owned()));
        }

        let tweet_id = url_vec[url_vec.len() - 1];
        evidence_map.insert(
            "tweetId".to_string(),
            serde_json::Value::String(tweet_id.to_owned()),
        );
        let evidence = Evidence {
            id: None,
            type_: vec!["TwitterVerificationPublicTweet".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject.did()?,
            "sameAs": format!("{}{}", "https://twitter.com/",  self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "TwitterVerification".to_string(),
        ])
    }
}
