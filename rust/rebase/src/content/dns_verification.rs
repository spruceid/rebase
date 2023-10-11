use crate::types::{
    defs::{Content, Subject},
    enums::subject::Subjects,
    error::ContentError,
};
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use std::collections::HashMap;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, JsonSchema, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DnsVerificationContent {
    pub domain: String,
    pub subject: Subjects,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Content for DnsVerificationContent {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            "https://spec.rebase.xyz/contexts/v1",
            "https://schema.org/"
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = HashMap::new();

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        evidence_map.insert(
            "dnsServer".to_string(),
            serde_json::Value::String("https://cloudflare-dns.com/dns-query".to_string()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["DnsVerificationMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject.did()?,
            "sameAs": format!("dns:{}",  self.domain)
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "DnsVerification".to_string(),
        ])
    }
}
