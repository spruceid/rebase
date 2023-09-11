mod utils;
use crate::utils::set_panic_hook;
use js_sys::Promise;
use rebase_witness_sdk::{
    client::{Client as RebaseClient, DelegatedAttestationConfig, Endpoints},
    types::{
        AttestationStatement, AttestationTypes, InstructionsReq, Proofs, SessionConfig, Statements,
        VCWrapper, JWK,
    },
};
use serde_json::from_str;
use std::{str, sync::Arc};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

// Dead-simple debug.
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
impl WasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> Result<WasmClient, String> {
        set_panic_hook();
        let c = from_str::<RebaseClient>(config).map_err(|e| e.to_string())?;
        is_valid(&c.endpoints)?;
        Ok(WasmClient {
            client: Arc::new(c),
        })
    }

    pub async fn new_jwk(&self) -> Promise {
        future_to_promise(async move {
            let key = JWK::generate_ed25519()
                .map_err(|error| format!("failed to generate session key: {}", error))?;

            Ok(serde_json::to_string(&key)
                .map_err(|e| format!("failed to make JWK string: {}", e))?
                .into())
        })
    }

    pub async fn siwe_message(
        &self,
        session_config: String,
        service_key: String,
        delegated_capabilities: String,
    ) -> Promise {
        future_to_promise(async move {
            let session_config: SessionConfig =
                from_str(&session_config).map_err(|e| e.to_string())?;
            let delegated_capabilities: Vec<AttestationTypes> =
                from_str(&delegated_capabilities).map_err(|e| e.to_string())?;
            let dac = jserr!(
                RebaseClient::siwe_message(session_config, &service_key, &delegated_capabilities)
                    .await
            );

            Ok(jserr!(serde_json::to_string(&dac)).into())
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

    pub fn delegated_attestation_jwt(
        &self,
        delegated_attestation_config: String,
        statement: String,
    ) -> Promise {
        let client = self.client.clone();
        future_to_promise(async move {
            let delegated_attestation_config: DelegatedAttestationConfig =
                jserr!(serde_json::from_str(&delegated_attestation_config));
            let statement: AttestationStatement = jserr!(serde_json::from_str(&statement));
            let res = jserr!(
                client
                    .delegated_attestation_jwt(delegated_attestation_config, statement)
                    .await
            );
            Ok(jserr!(serde_json::to_string(&res)).into())
        })
    }
}
