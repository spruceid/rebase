extern crate rebase;

use rebase::schema::self_signed::{default_self_signed_credential, verify_inner_signature};
use serde_json::to_string;
use ssi::jwk::JWK;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use tokio;

mod util;

fn get_key1() -> Result<JWK, String> {
    util::get_key("./examples/temp/ed25519_self_signed/keys/key1.jwk")
}

fn get_key2() -> Result<JWK, String> {
    util::get_key("./examples/temp/ed25519_self_signed/keys/key2.jwk")
}

fn fmt_did1(url: &str) -> Result<(), String> {
    util::fmt_did(
        "./examples/temp/ed25519_self_signed/serve/key1/.well-known/did.json",
        url,
    )
}

fn fmt_did2(url: &str) -> Result<(), String> {
    util::fmt_did(
        "./examples/temp/ed25519_self_signed/serve/key2/.well-known/did.json",
        url,
    )
}

#[tokio::main]
async fn main() {
    let url1 = env::args().skip(1).next().unwrap();
    let url2 = env::args().skip(2).next().unwrap();

    let key1 = get_key1().unwrap();
    let key2 = get_key2().unwrap();

    fmt_did1(&url1).unwrap();
    fmt_did2(&url2).unwrap();

    let id1 = format!("did:web:{}", &url1);
    let id2 = format!("did:web:{}", &url2);

    let signer1 =
        rebase::signer::ed25519::Ed25519DidWebJwk::new(id1, key1, "controller".to_string())
            .await
            .unwrap();

    let signer2 =
        rebase::signer::ed25519::Ed25519DidWebJwk::new(id2.clone(), key2, "controller".to_string())
            .await
            .unwrap();

    let credential = default_self_signed_credential(&signer1, &signer2)
        .await
        .unwrap();

    let s = to_string(&credential).unwrap();

    println!("{}", s);

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./examples/temp/ed25519_self_signed/credentials/vc.json")
        .unwrap();

    f.write_all(s.as_bytes()).unwrap();

    // TODO: Inspect inner Credential to validate inner siganture.
    println!("TODO: validate inner signature");

    verify_inner_signature(credential).await.unwrap();
}
