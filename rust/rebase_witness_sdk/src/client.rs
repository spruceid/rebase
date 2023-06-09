use crate::types::{
    handle_verify, CredentialWrapper, InstructionsReq, JWTWrapper, Proofs, Statements, VCWrapper,
    VerifyRes,
};
use rebase::types::defs::{ResolverOpts, StatementResponse};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;
use ts_rs::TS;
use url::Url;

#[derive(Debug, Deserialize, Error, Serialize)]
pub enum ClientError {
    #[error("failed in configuration: {0}")]
    Config(String),
    #[error("failed in jwt witness: {0}")]
    JWT(String),
    #[error("failed in statement generation: {0}")]
    Statement(String),
    #[error("failed in ld generation: {0}")]
    Ld(String),
}

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Endpoints {
    #[ts(type = "string", optional)]
    pub witness_jwt: Option<Url>,
    #[ts(type = "string", optional)]
    pub witness_ld: Option<Url>,
    #[ts(type = "string")]
    pub statement: Url,
    #[ts(type = "string")]
    pub instructions: Url,
    #[ts(type = "string", optional)]
    pub verify: Option<Url>,
}

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(rename = "ClientConfig")]
#[ts(export)]
pub struct Client {
    pub endpoints: Endpoints,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_opts: Option<ResolverOpts>,
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
struct WitnessErr {
    pub error: String,
}

impl Client {
    pub fn new(
        endpoints: Endpoints,
        resolver_opts: Option<ResolverOpts>,
    ) -> Result<Client, ClientError> {
        if endpoints.witness_jwt.is_none() && endpoints.witness_ld.is_none() {
            return Err(ClientError::Config("No witness url found".to_string()));
        };

        Ok(Client {
            endpoints,
            resolver_opts,
        })
    }

    pub async fn instructions(
        &self,
        req: InstructionsReq,
    ) -> Result<serde_json::Value, ClientError> {
        let client = HttpClient::new();

        let res = client
            .post(self.endpoints.instructions.clone())
            .json(&req)
            .send()
            .await
            .map_err(|e| ClientError::Statement(e.to_string()))?
            .json()
            .await
            .map_err(|e| ClientError::Statement(e.to_string()))?;

        Ok(res)
    }

    pub async fn statement(&self, req: Statements) -> Result<StatementResponse, ClientError> {
        let client = HttpClient::new();

        let res = client
            .post(self.endpoints.statement.clone())
            .json(&req)
            .send()
            .await
            .map_err(|e| ClientError::Statement(e.to_string()))?;

        match res.json::<serde_json::Value>().await {
            Err(e) => Err(ClientError::Statement(e.to_string())),
            Ok(val) => match serde_json::from_value::<StatementResponse>(val.clone()) {
                Ok(r) => Ok(r),
                Err(p) => match serde_json::from_value::<WitnessErr>(val) {
                    Err(_) => Err(ClientError::Statement(p.to_string())),
                    Ok(w) => Err(ClientError::Statement(w.error)),
                },
            },
        }
    }

    pub async fn witness_jwt(&self, req: Proofs) -> Result<JWTWrapper, ClientError> {
        match &self.endpoints.witness_jwt {
            Some(endpoint) => {
                let client = HttpClient::new();

                let res = client
                    .post(endpoint.clone())
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| ClientError::JWT(e.to_string()))?;

                match res.json::<serde_json::Value>().await {
                    Err(e) => Err(ClientError::JWT(e.to_string())),
                    Ok(val) => match serde_json::from_value::<JWTWrapper>(val.clone()) {
                        Ok(r) => Ok(r),
                        Err(p) => match serde_json::from_value::<WitnessErr>(val) {
                            Err(_) => Err(ClientError::JWT(p.to_string())),
                            Ok(w) => Err(ClientError::JWT(w.error)),
                        },
                    },
                }
            }
            None => Err(ClientError::JWT("No configured JWT endpoint".to_string())),
        }
    }

    pub async fn witness_ld(&self, req: Proofs) -> Result<CredentialWrapper, ClientError> {
        match &self.endpoints.witness_ld {
            Some(endpoint) => {
                let client = HttpClient::new();

                let res = client
                    .post(endpoint.clone())
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| ClientError::Ld(e.to_string()))?;

                match res.json::<serde_json::Value>().await {
                    Err(e) => Err(ClientError::Ld(e.to_string())),
                    Ok(val) => match serde_json::from_value::<CredentialWrapper>(val.clone()) {
                        Ok(r) => Ok(r),
                        Err(p) => match serde_json::from_value::<WitnessErr>(val) {
                            Err(_) => Err(ClientError::Ld(p.to_string())),
                            Ok(w) => Err(ClientError::Ld(w.error)),
                        },
                    },
                }
            }
            None => Err(ClientError::Ld("No configured LD endpoint".to_string())),
        }
    }

    pub async fn verify(&self, req: VCWrapper) -> Result<VerifyRes, ClientError> {
        Ok(VerifyRes {
            success: matches!(handle_verify(&req, &self.resolver_opts).await, Ok(_)),
        })
    }

    pub async fn witness_verify(&self, req: VCWrapper) -> Result<VerifyRes, ClientError> {
        match &self.endpoints.verify {
            Some(endpoint) => {
                let client = HttpClient::new();

                let res = client
                    .post(endpoint.clone())
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| ClientError::Ld(e.to_string()))?;

                match res.json::<serde_json::Value>().await {
                    Err(e) => Err(ClientError::Ld(e.to_string())),
                    Ok(val) => match serde_json::from_value::<VerifyRes>(val.clone()) {
                        Ok(r) => Ok(r),
                        Err(p) => match serde_json::from_value::<WitnessErr>(val) {
                            Err(_) => Err(ClientError::Ld(p.to_string())),
                            Ok(w) => Err(ClientError::Ld(w.error)),
                        },
                    },
                }
            }
            None => Err(ClientError::Ld(
                "No configured verify LD endpoint".to_string(),
            )),
        }
    }
}
