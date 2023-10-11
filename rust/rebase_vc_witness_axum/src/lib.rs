// TODO: More robust err handling via anyhow err?
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use rebase_witness_sdk::types::{
    handle_verify, issuer::ed25519::Ed25519Jwk, InstructionsReq, Proofs, Statements, VCWrapper,
    WitnessFlow,
};
use serde::Deserialize;
use serde_json::Value;
use std::{ops::Deref, sync::Arc};

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub rebase: WitnessFlow,
    pub issuer: Ed25519Jwk,
}

// TODO: Change the calls so the return value is well typed, here it would be: Json<Instructions> instead of Json<Value> (???)
// TODO: Pass through err messages?
pub async fn instructions_handler(
    State(state): State<Arc<Config>>,
    instructions_req: Json<InstructionsReq>,
) -> Result<Json<Value>, StatusCode> {
    if let Ok(v) = state.rebase.handle_instructions(&instructions_req).await {
        Ok(Json(v))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn statement_handler(
    State(state): State<Arc<Config>>,
    statement_req: Json<Statements>,
) -> Result<Json<Value>, StatusCode> {
    let req = statement_req.deref();
    if let Ok(v) = state
        .rebase
        .handle_statement(req.to_owned(), state.issuer.clone())
        .await
    {
        Ok(Json(v))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn witness_ld_handler(
    State(state): State<Arc<Config>>,
    witness_ld_req: Json<Proofs>,
) -> Result<Json<Value>, StatusCode> {
    let req = witness_ld_req.deref();
    if let Ok(v) = state
        .rebase
        .handle_ld(req.to_owned(), state.issuer.clone())
        .await
    {
        Ok(Json(v))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn witness_jwt_handler(
    State(state): State<Arc<Config>>,
    witness_jwt_req: Json<Proofs>,
) -> Result<Json<Value>, StatusCode> {
    let req = witness_jwt_req.deref();
    if let Ok(v) = state
        .rebase
        .handle_jwt(req.to_owned(), state.issuer.clone())
        .await
    {
        Ok(Json(v))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn verify_credential_handler(
    State(_state): State<Arc<Config>>,
    verify_req: Json<VCWrapper>,
) -> Result<Json<Value>, StatusCode> {
    let req = verify_req.deref();
    if handle_verify(req, &None).await.is_ok() {
        Ok(Json(serde_json::json!({"success": true})))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn service(config: Config) -> Router {
    let state = Arc::new(config);

    Router::new()
        .route("/instructions", post(instructions_handler))
        .route("/statement", post(statement_handler))
        .route("/witness_ld", post(witness_ld_handler))
        .route("/witness_jwt", post(witness_jwt_handler))
        .route("/verify", post(verify_credential_handler))
        .with_state(state)
}
