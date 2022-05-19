use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct Did {
    #[serde(rename = "@context")]
    pub context: Vec<Context>,
    pub id: String,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub controller: String,
    #[serde(rename = "publicKeyJwk")]
    pub key: VerificationPubKey,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationPubKey {
    pub kty: String,
    pub crv: String,
    pub x: String,
}

#[derive(Deserialize, Serialize)]
pub struct ContextKey {
    #[serde(rename = "Ed25519VerificationKey2018")]
    pub verification_key: String,
    #[serde(rename = "publicKeyJwk")]
    pub public_key: ContextPubKey,
}

#[derive(Deserialize, Serialize)]
pub struct ContextPubKey {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
    String(String),
    Struct(ContextKey),
}

pub fn get_key(path: &str) -> Result<String, String> {
    let mut f = File::open(path).map_err(|e| format!("{}", e))?;
    let mut c = String::new();
    f.read_to_string(&mut c).map_err(|e| format!("{}", e))?;
    // Ok(serde_json::from_str(&c).map_err(|e| format!("{}", e))?)
    Ok(c)
}

pub fn fmt_did(path: &str, url: &str) -> Result<(), String> {
    let id = format!("did:web:{}", url);
    let suffixed = format!("{}#controller", &id);

    let mut s = String::new();
    {
        let mut f = File::open(path).map_err(|e| format!("{}", e))?;
        f.read_to_string(&mut s).map_err(|e| format!("{}", e))?;
    }

    let mut raw: Did = serde_json::from_str(&s).map_err(|e| format!("{}", e))?;
    raw.id = id.clone();
    raw.verification_method[0].id = suffixed.clone();
    raw.verification_method[0].controller = id;
    raw.authentication[0] = suffixed.clone();
    raw.assertion_method[0] = suffixed;

    let str = serde_json::to_string(&raw).map_err(|e| format!("{}", e))?;
    let mut f = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("{}", e))?;

    f.write_all(str.as_bytes()).map_err(|e| format!("{}", e))
}

fn main() {}
