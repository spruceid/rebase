mod utils;

use js_sys::Promise;
use rebase_witness_sdk::{
    client::{Client as RebaseClient, Endpoints},
    types::{InstructionsReq, StatementReq, VerifyJWTReq, VerifyLDReq, WitnessReq},
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
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
pub struct WasmClient {
    client: Arc<RebaseClient>,
}

#[wasm_bindgen]
#[derive(Deserialize, Serialize)]
pub struct Config {
    instructions: String,
    statement: String,
    jwt: Option<String>,
    ld: Option<String>,
    verify_jwt: Option<String>,
    verify_ld: Option<String>,
}

impl Config {
    fn is_valid(&self) -> Result<(), String> {
        if self.jwt.is_none() && self.ld.is_none() {
            Err("At least one of JWT or LD urls must be set".to_string())
        } else {
            Ok(())
        }
    }
}

#[wasm_bindgen]
impl WasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> Result<WasmClient, String> {
        let config: Config = from_str(config).map_err(|e| e.to_string())?;
        config.is_valid()?;

        let jwt: Option<Url> = match config.jwt {
            Some(s) => Some(Url::parse(&s).map_err(|e| e.to_string())?),
            None => None,
        };

        let verify_jwt: Option<Url> = match config.verify_jwt {
            Some(s) => Some(Url::parse(&s).map_err(|e| e.to_string())?),
            None => None,
        };

        let ld: Option<Url> = match config.ld {
            Some(s) => Some(Url::parse(&s).map_err(|e| e.to_string())?),
            None => None,
        };

        let verify_ld: Option<Url> = match config.verify_ld {
            Some(s) => Some(Url::parse(&s).map_err(|e| e.to_string())?),
            None => None,
        };

        let statement = Url::parse(&config.statement).map_err(|e| e.to_string())?;
        let instructions = Url::parse(&config.instructions).map_err(|e| e.to_string())?;
        Ok(WasmClient {
            client: Arc::new(
                RebaseClient::new(Endpoints {
                    jwt,
                    ld,
                    statement,
                    instructions,
                    verify_jwt,
                    verify_ld,
                })
                .map_err(|e| e.to_string())?,
            ),
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
            let req: StatementReq = jserr!(serde_json::from_str(&req));
            // TODO: Work from here to extricate witness error, if exists.
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

    pub fn verify_jwt(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: VerifyJWTReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.verify_jwt(req).await);
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

    pub fn verify_ld(&self, req: String) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let req: VerifyLDReq = jserr!(serde_json::from_str(&req));
            let res = jserr!(client.verify_ld(req).await);
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }
}
