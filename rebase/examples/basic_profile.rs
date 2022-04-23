extern crate rebase;

use rebase::schema::schema_type::SchemaType;
use serde_json::to_string;
use ssi::jwk::JWK;
use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let key_path = env::args().skip(1).next().unwrap();

    let key = key_from_path(key_path).unwrap();

    let id = "did:web:example.com".to_string();
    let signer = rebase::signer::ed25519::Ed25519::new(
        id,
        key,
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

fn key_from_path(path: String) -> Result<JWK, String> {
    Err("Unimplemented!".to_string())
}
