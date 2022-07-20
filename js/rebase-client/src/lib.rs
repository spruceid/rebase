mod utils;

use rebase_witness_sdk::client::{Client, Endpoints};
use url::Url;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let endpoints = Endpoints {
        jwt: Some(Url::parse("http://localhost:8787/witness").unwrap()),
        ld: None,
        statement: Url::parse("http://localhost:8787/statement").unwrap(),
    };

    match Client::new(endpoints) {
        Ok(_) => alert("The client initialized!"),
        Err(e) => alert(&format!("Oh no: {}", e.to_string())),
    };
}
