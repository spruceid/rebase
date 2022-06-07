extern crate rebase;

use rebase::schema::schema_type::SchemaType;
use rebase::signer::signer::Signer;
use serde_json::to_string;
use std::env;
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};
use tokio;

mod util;

fn get_key() -> Result<String, String> {
    util::get_key("./examples/temp/ed25519_basic/keys/controller.jwk")
}

fn fmt_did(url: &str) -> Result<(), String> {
    util::fmt_did("./examples/temp/ed25519_basic/.well-known/did.json", url)
}

fn get_line() -> Result<String, String> {
    let _ = stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).map_err(|e| format!("{}", e))?;

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    Ok(s)
}

#[tokio::main]
async fn main() {
    let url = env::args().skip(1).next().unwrap();
    let id = format!("did:web:{}", &url);

    let key = get_key().unwrap();
    fmt_did(&url).unwrap();

    let signer = rebase::signer::ed25519::Ed25519DidWebJwk::new(&id, &key, "controller")
        .await
        .unwrap();

    println!("Let's make a post, then save it out as a Verifiable Credential!");
    println!("Enter the title of your post:");

    let title = get_line().unwrap();

    println!("Good, now for the body of the post:");

    let body = get_line().unwrap();

    println!("Getting subj ID");
    let subject_id = signer.did_id().unwrap();

    println!("About to get schema");
    let schema = rebase::schema::basic_post::BasicPost {
        title,
        body,
        subject_id,
    };

    let credential = schema.credential(&signer).await.unwrap();
    let s = to_string(&credential).unwrap();

    println!("{}", s);
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./examples/temp/ed25519_basic/credentials/vc.json")
        .unwrap();

    f.write_all(s.as_bytes()).unwrap()
}
