use crate::types::{
    InstructionsReq, StatementReq, VerifyJWTReq, VerifyLDReq, VerifyRes, WitnessJWTRes,
    WitnessLDRes, WitnessReq,
};
use rebase::types::defs::FlowResponse;
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
    pub verify_jwt: Option<Url>,
    pub verify_ld: Option<Url>,
}

#[derive(Clone)]
pub struct Client {
    endpoints: Endpoints,
}

// TODO: Make this more explicit higher up so that this is less of a pinky promise
// and actually type enforced.
#[derive(Deserialize, Serialize)]
struct WitnessErr {
    error: String,
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

    pub async fn statement(&self, req: StatementReq) -> Result<FlowResponse, ClientError> {
        let client = HttpClient::new();

        // let res: FlowResponse = client
        let res = client
            .post(self.endpoints.statement.clone())
            .json(&req)
            .send()
            .await
            .map_err(|e| ClientError::Statement(e.to_string()))?;

        match res.json::<FlowResponse>().await {
            Ok(r) => Ok(r),
            Err(e) => match res.json::<WitnessErr>().await {
                Ok(w) => Err(ClientError::Statement(w.error)),
                Err(_) => Err(ClientError::Statement(e.to_string())),
            },
        }
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

    // TODO: Unify these when making the request a single enum.
    pub async fn verify_jwt(&self, req: VerifyJWTReq) -> Result<VerifyRes, ClientError> {
        match &self.endpoints.verify_jwt {
            Some(endpoint) => {
                let client = HttpClient::new();

                let res: VerifyRes = client
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
            None => Err(ClientError::Ld(
                "No configured verify JWT endpoint".to_string(),
            )),
        }
    }

    // TODO: Unify these when making the request a single type enum.
    pub async fn verify_ld(&self, req: VerifyLDReq) -> Result<VerifyRes, ClientError> {
        match &self.endpoints.verify_ld {
            Some(endpoint) => {
                let client = HttpClient::new();

                let res: VerifyRes = client
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
            None => Err(ClientError::Ld(
                "No configured verify LD endpoint".to_string(),
            )),
        }
    }
}
