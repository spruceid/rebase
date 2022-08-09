extern crate wasm_bindgen;

extern crate log;
use js_sys::Promise;

use rebase::signer::ed25519::Ed25519DidWebJwk;
use rebase_witness_sdk::witness::{
    statement as handle_statement, 
    instructions as handle_instructions, 
    witness_jwt as handle_jwt, 
    InstructionReq, 
    StatementReq,
    WitnessGenerator, 
    WitnessOpts,
    WitnessReq,
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

// TODO: Make passed from worker
const SPRUCE_DIDWEB: &str = "did:web:rebasedemokey.pages.dev";

// TODO: Make generator take opts enum/struct?
pub fn create_generator(opts: WitnessOpts) -> WitnessGenerator {
    WitnessGenerator::new(opts)
}

#[wasm_bindgen]
pub fn instructions(req: String) -> Promise {
    future_to_promise(async move {
        let req: InstructionReq = jserr!(serde_json::from_str(&req));
        let res = jserr!(handle_instructions(req));
        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}

#[wasm_bindgen]
pub async fn statement(req: String) -> Promise {
    future_to_promise(async move {
        let req: StatementReq = jserr!(serde_json::from_str(&req));
        let res = jserr!(handle_statement(req).await);
        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}

#[wasm_bindgen]
pub async fn witness(secret: String, witness_request: String, opts: String) -> Promise {
    future_to_promise(async move {
        let opts: WitnessOpts = jserr!(serde_json::from_str(&opts));
        let signer = jserr!(Ed25519DidWebJwk::new(SPRUCE_DIDWEB, &secret, "controller").await);
        let witness_request: WitnessReq = jserr!(serde_json::from_str(&witness_request));
        let generator = create_generator(opts);
        let res = jserr!(handle_jwt(witness_request, &generator, &signer).await);

        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}
