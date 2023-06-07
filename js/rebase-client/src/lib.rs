mod utils;

use js_sys::Promise;
use rebase_witness_sdk::{
    client::{Client as RebaseClient, Endpoints},
    types::{InstructionsReq, Proofs, Statements, VCWrapper},
};
use serde_json::from_str;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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

#[wasm_bindgen]
pub struct WasmClient {
    client: Arc<RebaseClient>,
}

fn is_valid(endpoints: &Endpoints) -> Result<(), String> {
    if endpoints.witness_jwt.is_none() && endpoints.witness_ld.is_none() {
        Err("At least one of JWT or LD witness urls must be set".to_string())
    } else {
        Ok(())
    }
}

#[wasm_bindgen]
impl WasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> Result<WasmClient, String> {
        let c = from_str::<RebaseClient>(config).map_err(|e| e.to_string())?;
        is_valid(&c.endpoints)?;
        Ok(WasmClient {
            client: Arc::new(c),
        })
    }

    pub fn instructions(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: InstructionsReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.instructions(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn statement(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: Statements = jserr!(serde_json::from_str(&req));
            // TODO: Work from here to extricate witness error, if exists.
            let res = jserr!(client.statement(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn witness_jwt(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: Proofs = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.witness_jwt(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn witness_ld(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: Proofs = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.witness_ld(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn witness_verify(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: VCWrapper = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.witness_verify(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn verify(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: VCWrapper = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.verify(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }
}
