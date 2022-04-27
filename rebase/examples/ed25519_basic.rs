extern crate rebase;

use rebase::schema::schema_type::SchemaType;
use serde_json::to_string;
use ssi::jwk::JWK;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio;

#[tokio::main]
async fn main() {
    let key_path = env::args().skip(1).next().unwrap();

    let key = key_from_path(&key_path).unwrap();

    // TODO: Change to a passed in variable to support local hosting
    let id = "did:web:41f6-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io".to_string();

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

    println!("{}", to_string(&credential).unwrap())
}

fn key_from_path(path: &str) -> Result<JWK, String> {
    let mut f = File::open(path).map_err(|e| format!("{}", e))?;
    let mut c = String::new();
    f.read_to_string(&mut c).map_err(|e| format!("{}", e))?;
    Ok(serde_json::from_str(&c).map_err(|e| format!("{}", e))?)
}
