extern crate wasm_bindgen;
extern crate log;
use js_sys::Promise;

use rebase_witness_sdk::types::{
    issuer::ed25519::DidWebJwk,
    InstructionsReq, 
    StatementReq,
    WitnessReq,
    WitnessFlow,
};

use serde_json;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

macro_rules! jserr {
    ($expression:expr) => {
        match $expression {
            Ok(a) => a,
            Err(e) => {
                return Err(JsValue::from(format!("{}", e)));
            }
        }
    };
}

use wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(serde::Deserialize)]
pub struct Opts {
    witness: WitnessFlow,
    did: String
}

#[wasm_bindgen]
pub async fn instructions(req: String, opts: String) -> Promise {
    future_to_promise(async move {
        let req: InstructionsReq = jserr!(serde_json::from_str(&req));
        let opts: Opts = jserr!(serde_json::from_str(&opts));
        let res = jserr!(opts.witness.handle_instructions(&req).await);
        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}

#[wasm_bindgen]
pub async fn statement(secret: String, req: String, opts: String) -> Promise {
    future_to_promise(async move {
        let opts: Opts = jserr!(serde_json::from_str(&opts));
        let issuer = jserr!(DidWebJwk::new(&opts.did, &secret, "controller"));
        let req: StatementReq = jserr!(serde_json::from_str(&req));
        let res = jserr!(opts.witness.handle_statement(&req, &issuer).await);
        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}

#[wasm_bindgen]
pub async fn witness(secret: String, witness_request: String, opts: String) -> Promise {
    future_to_promise(async move {
        let opts: Opts = jserr!(serde_json::from_str(&opts));
        let issuer = jserr!(DidWebJwk::new(&opts.did, &secret, "controller"));
        let witness_request: WitnessReq = jserr!(serde_json::from_str(&witness_request));
        let res = jserr!(opts.witness.handle_jwt(&witness_request, &issuer).await);

        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}
