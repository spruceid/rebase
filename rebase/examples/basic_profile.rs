extern crate rebase;
use std::env;

fn main() {
    let key_path = env::args().skip(1).next();

    // let key = key_from_path(key_path).unwrap();
    // let id = "did:web:example.com".to_string();
    // let signer = rebase::signer::ed25519::Ed25519::new(key, id);

    let schema = rebase::schema::basic_profile::BasicProfile {
        alias: "foo".to_string(),
        description: "bar".to_string(),
        website: "https://www.example.com".to_string(),
        logo: "example.jpg".to_string(),
    };

    // let credential = schema.credential(&signer).unwrap();

    println!("Hello Keypath: {}", key_path.unwrap())
    // println!("{}", credential)
}