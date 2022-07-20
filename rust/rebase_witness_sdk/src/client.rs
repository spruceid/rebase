use crate::witness::{StatementReq, StatementRes, WitnessJWTRes, WitnessLDRes, WitnessReq};
use reqwest::Client as HttpClient;
use thiserror::Error;
use url::Url;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Error, Debug)]
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

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Endpoints {
    pub jwt: Option<Url>,
    pub ld: Option<Url>,
    pub statement: Url,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Client {
    endpoints: Endpoints,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Client {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(endpoints: Endpoints) -> Result<Client, ClientError> {
        if endpoints.jwt.is_none() && endpoints.ld.is_none() {
            return Err(ClientError::Config("No witness url found".to_string()));
        };

        Ok(Client { endpoints })
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

                let res: WitnessJWTRes = client
                    .post(endpoint.clone())
                    .json(&req)
                    .send()
                    .await
                    .map_err(|e| ClientError::JWT(e.to_string()))?
                    .json()
                    .await
                    .map_err(|e| ClientError::JWT(e.to_string()))?;

                Ok(res)
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

#[cfg(test)]
mod tests {
    use super::*;
    use rebase::{util::util::*, witness::witness::Statement, witness::*};

    fn new_client() -> Result<Client, String> {
        // TODO: Update to use a worker that supports LD routes.
        let endpoints = Endpoints {
            jwt: Some(Url::parse("http://localhost:8787/witness").unwrap()),
            ld: None,
            statement: Url::parse("http://localhost:8787/statement").unwrap(),
        };

        Client::new(endpoints).map_err(|e| e.to_string())
    }

    async fn check_statement(
        client: &Client,
        opts: statement_type::StatementTypes,
        statement: &str,
    ) -> Result<(), String> {
        let req = StatementReq { opts };

        let res = client.statement(req).await.unwrap();

        if res.statement != statement {
            return Err(format!(
                "Expected matching statements, got: {} AND {}",
                res.statement, statement
            ));
        }

        Ok(())
    }

    // NOTE: This requires a demo worker to be running at
    // localhost:8787. This is achievable from rebase/demo/worker
    // using
    // $ wrangler dev

    // NOTE: This expects tzprofiles.dev to actually have
    // the DNS record it is looking for
    #[tokio::test]
    async fn test_dns() {
        let client = new_client().unwrap();

        let did = test_eth_did();

        let opts = dns::Claim {
            domain: "tzprofiles.dev".to_owned(),
            prefix: "rebase_sig=".to_owned(),
            key_type: did,
        };

        let statement = opts.generate_statement().unwrap();

        check_statement(
            &client,
            statement_type::StatementTypes::Dns(opts.clone()),
            &statement,
        )
        .await
        .unwrap();

        let req = WitnessReq {
            proof: proof_type::ProofTypes::Dns(opts),
        };

        client.jwt(req).await.unwrap();
    }

    // NOTE: This expects the given gist to actually have
    // the signature it is looking for
    #[tokio::test]
    async fn test_github() {
        let client = new_client().unwrap();

        let did = test_eth_did();
        let opts = github::Opts {
            handle: "krhoda".to_string(),
            key_type: did,
        };

        let statement = opts.generate_statement().unwrap();

        check_statement(
            &client,
            statement_type::StatementTypes::GitHub(opts.clone()),
            &statement,
        )
        .await
        .unwrap();

        let proof = github::Claim {
            gist_id: "28fb83438a26e70350ef3195d999882d".to_string(),
            statement_opts: opts,
        };

        let req = WitnessReq {
            proof: proof_type::ProofTypes::GitHub(proof),
        };

        client.jwt(req).await.unwrap();
    }

    // NOTE: This expects the given tweet to actually have
    // the signature it is looking for
    #[tokio::test]
    async fn test_twitter() {
        let client = new_client().unwrap();

        let did = test_eth_did();
        let opts = twitter::Opts {
            handle: "evalapplyquote".to_string(),
            key_type: did,
        };

        let statement = opts.generate_statement().unwrap();

        check_statement(
            &client,
            statement_type::StatementTypes::Twitter(opts.clone()),
            &statement,
        )
        .await
        .unwrap();

        let proof = twitter::Claim {
            tweet_url: "https://twitter.com/evalapplyquote/status/1542901885815820288".to_string(),
            statement_opts: opts,
        };

        let req = WitnessReq {
            proof: proof_type::ProofTypes::Twitter(proof),
        };

        client.jwt(req).await.unwrap();
    }

    #[tokio::test]
    async fn test_self_signed() {
        let client = new_client().unwrap();

        let did = test_eth_did();
        let did2 = test_eth_did_2();

        let opts = self_signed::Opts {
            key_1: did,
            key_2: did2,
        };

        let statement = opts.generate_statement().unwrap();

        check_statement(
            &client,
            statement_type::StatementTypes::SelfSigned(opts.clone()),
            &statement,
        )
        .await
        .unwrap();

        let proof = self_signed::Claim::new(
            opts,
            TEST_2KEY_ETH_SIG_1.to_owned(),
            TEST_2KEY_ETH_SIG_2.to_owned(),
        )
        .await
        .unwrap();

        let req = WitnessReq {
            proof: proof_type::ProofTypes::SelfSigned(proof),
        };

        client.jwt(req).await.unwrap();
    }
}
