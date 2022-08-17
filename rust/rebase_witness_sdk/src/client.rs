use crate::witness::{
    InstructionReq, StatementReq, StatementRes, WitnessJWTRes, WitnessLDRes, WitnessReq,
};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;
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

#[derive(Clone)]
pub struct Endpoints {
    pub jwt: Option<Url>,
    pub ld: Option<Url>,
    pub statement: Url,
    pub instructions: Url,
}

#[derive(Clone)]
pub struct Client {
    endpoints: Endpoints,
}

impl Client {
    pub fn new(endpoints: Endpoints) -> Result<Client, ClientError> {
        if endpoints.jwt.is_none() && endpoints.ld.is_none() {
            return Err(ClientError::Config("No witness url found".to_string()));
        };

        Ok(Client { endpoints })
    }

    pub async fn instructions(
        &self,
        req: InstructionReq,
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

    pub async fn statement(&self, req: StatementReq) -> Result<StatementRes, ClientError> {
        let client = HttpClient::new();

        let res: StatementRes = client
            .post(self.endpoints.statement.clone())
            .json(&req)
            .send()
            .await
            .map_err(|e| ClientError::Statement(e.to_string()))?
            .json()
            .await
            .map_err(|e| ClientError::Statement(e.to_string()))?;

        Ok(res)
    }

    pub async fn jwt(&self, req: WitnessReq) -> Result<WitnessJWTRes, ClientError> {
        match &self.endpoints.jwt {
            Some(endpoint) => {
                let client = HttpClient::new();

                client
                    .post(endpoint.clone())
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| ClientError::JWT(e.to_string()))?
                    .json::<WitnessJWTRes>()
                    .await
                    .map_err(|e| ClientError::JWT(e.to_string()))
            }
            None => Err(ClientError::JWT("No configured JWT endpoint".to_string())),
        }
    }

    pub async fn ld(&self, req: WitnessReq) -> Result<WitnessLDRes, ClientError> {
        match &self.endpoints.ld {
            Some(endpoint) => {
                let client = HttpClient::new();

                let res: WitnessLDRes = client
                    .post(endpoint.clone())
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| ClientError::Ld(e.to_string()))?
                    .json()
                    .await
                    .map_err(|e| ClientError::Ld(e.to_string()))?;

                Ok(res)
            }
            None => Err(ClientError::Ld("No configured LD endpoint".to_string())),
        }
    }
}
