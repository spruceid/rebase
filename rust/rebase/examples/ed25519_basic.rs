extern crate rebase;

use rebase::schema::schema_type::SchemaType;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use ssi::jwk::JWK;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use tokio;

#[derive(Deserialize, Serialize)]
struct Did {
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
struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub controller: String,
    #[serde(rename = "publicKeyJwk")]
    pub key: VerificationPubKey,
}

#[derive(Deserialize, Serialize)]
struct VerificationPubKey {
    pub kty: String,
    pub crv: String,
    pub x: String,
}

#[derive(Deserialize, Serialize)]
struct ContextKey {
    #[serde(rename = "Ed25519VerificationKey2018")]
    pub verification_key: String,
    #[serde(rename = "publicKeyJwk")]
    pub public_key: ContextPubKey,
}

#[derive(Deserialize, Serialize)]
struct ContextPubKey {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub _type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum Context {
    String(String),
    Struct(ContextKey),
}

fn get_key() -> Result<JWK, String> {
    let mut f = File::open("./examples/temp/ed25519_basic/keys/controller.jwk")
        .map_err(|e| format!("{}", e))?;
    let mut c = String::new();
    f.read_to_string(&mut c).map_err(|e| format!("{}", e))?;
    println!("{}", &c);
    Ok(serde_json::from_str(&c).map_err(|e| format!("{}", e))?)
}

fn fmt_did(url: String) -> Result<(), String> {
    let id = format!("did:web:{}", url);
    let suffixed = format!("{}#controller", &id);

    let mut s = String::new();
    {
        let mut f = File::open("./examples/temp/ed25519_basic/.well-known/did.json")
            .map_err(|e| format!("{}", e))?;
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
        .open("./examples/temp/ed25519_basic/.well-known/did.json")
        .map_err(|e| format!("{}", e))?;

    f.write_all(str.as_bytes()).map_err(|e| format!("{}", e))?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let url = env::args().skip(1).next().unwrap();
    let id = format!("did:web:{}", &url);

    let key = get_key().unwrap();
    fmt_did(url).unwrap();

    let signer = rebase::signer::ed25519::Ed25519::new(
        id,
        key,
        "controller".to_string(),
        rebase::signer::ed25519::SignerTypes::DIDWebJWK,
    )
    .await
    .unwrap();

    let schema = rebase::schema::basic_profile::BasicProfile {
        alias: "foo".to_string(),
        description: "bar".to_string(),
        website: "https://www.example.com".to_string(),
        logo: "example.jpg".to_string(),
    };

    let credential = schema.credential(&signer).await.unwrap();
    let s = to_string(&credential).unwrap();

    println!("{}", s);
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./examples/temp/ed25519_basic/credentials/vc.json")
        .map_err(|e| format!("{}", e))
        .unwrap();

    f.write_all(s.as_bytes()).unwrap()
}
