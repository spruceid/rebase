mod utils;

use rebase_witness_sdk::{
    client::{Client as RebaseClient, Endpoints},
    witness::{InstructionReq, StatementReq, WitnessReq},
};
// use serde_json::from_str;
use js_sys::Promise;
use std::sync::Arc;
use url::Url;
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
pub struct Client {
    client: Arc<RebaseClient>,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(
        instructions: String,
        statement: String,
        jwt: Option<String>,
        ld: Option<String>,
    ) -> Result<Client, String> {
        let jwt: Option<Url> = match jwt {
            Some(s) => Some(Url::parse(&s).map_err(|e| e.to_string())?),
            None => None,
        };

        let ld: Option<Url> = match ld {
            Some(s) => Some(Url::parse(&s).map_err(|e| e.to_string())?),
            None => None,
        };

        let statement = Url::parse(&statement).map_err(|e| e.to_string())?;
        let instructions = Url::parse(&instructions).map_err(|e| e.to_string())?;
        Ok(Client {
            client: Arc::new(
                RebaseClient::new(Endpoints {
                    jwt,
                    ld,
                    statement,
                    instructions,
                })
                .map_err(|e| e.to_string())?,
            ),
        })
    }

    pub fn instructions(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: InstructionReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.instructions(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn statement(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: StatementReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.statement(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn jwt(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: WitnessReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.jwt(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }

    pub fn ld(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: WitnessReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.ld(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }
}
