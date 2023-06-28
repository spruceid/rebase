use crate::types::{
    handle_verify, to_action, AttestationTypes, CredentialWrapper, InstructionsReq, JWTWrapper,
    Proofs, ResolverOpts, SessionConfig, StatementResponse, Statements, VCWrapper, VerifyRes, JWK,
};
use base64::{engine::general_purpose, Engine as _};
use rebase::{
    issuer::ed25519::Ed25519Jwk,
    proof::delegated_attestation::{parse_siwe_recap, DelegatedAttestationProof, ParsedReCap},
    statement::attestation::statement::AttestationStatement,
    types::{
        defs::{
            eip55, get_verification_method, DIDKey, DIDMethod, Issuer, Message, Source, Statement,
            Subject,
        },
        enums::{
            attestation::Attestation,
            subject::{Pkh, Subjects},
        },
    },
};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::BTreeMap, str::FromStr};
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
    #[error("failed in recap generation: {0}")]
    Capability(String),
    #[error("failed in delegated attestation config: {0}")]
    DelegatedConf(String),
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

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DelegatedAttestationPreConfig {
    service_key: String,
    session_config: SessionConfig,
    siwe_recap_message: String,
}

#[derive(Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DelegatedAttestationConfig {
    service_key: String,
    session_config: SessionConfig,
    siwe_recap_message: String,
    siwe_recap_signature: String,
}

impl DelegatedAttestationConfig {
    pub fn jwk(&self) -> Result<JWK, ClientError> {
        self.session_config
            .clone()
            .jwk
            .ok_or(ClientError::DelegatedConf(
                "No JWK in session config".to_string(),
            ))
    }

    pub async fn is_valid(&self) -> Result<ParsedReCap, ClientError> {
        // Is the SIWE Recap Valid?
        let p = parse_siwe_recap(&self.siwe_recap_message, &self.service_key)
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?;
        p.subject
            .valid_signature(&self.siwe_recap_message, &self.siwe_recap_signature)
            .await
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?;

        // Create the SIWE message to use a source of information for validation.
        let m = Message::from_str(&self.siwe_recap_message).map_err(|_e| {
            ClientError::DelegatedConf("Failed to parse ReCap into Message".to_string())
        })?;

        // Use SIWE's own validation to determine if the ReCap is valid from a timing perspective
        if !m.valid_now() {
            return Err(ClientError::DelegatedConf(
                "Capability is not valid at this time".to_string(),
            ));
        };

        Ok(p)
    }

    pub async fn is_jwk_valid(&self) -> Result<Ed25519Jwk, ClientError> {
        let j = self.jwk()?;
        let m = Message::from_str(&self.siwe_recap_message).map_err(|_e| {
            ClientError::DelegatedConf("Failed to parse ReCap into Message".to_string())
        })?;

        let expected_did = m.uri.to_string();
        let dk = DIDKey {};

        let d =
            // NOTE: This unwrap is safe from the above is_none check
            dk.generate(&Source::Key(&j))
                .ok_or(ClientError::DelegatedConf(
                    "DID Generation returned None".to_string(),
                ))?;

        let vm = get_verification_method(&d, &dk)
            .await
            .ok_or(ClientError::DelegatedConf(
                "Failed to generated verification method from DID".to_string(),
            ))?;

        if vm != expected_did {
            return Err(ClientError::DelegatedConf(format!(
                "Expected {} got {} for internal JWK session key",
                expected_did, vm
            )));
        }

        let split_did: Vec<String> = vm.split("#").map(|s| s.to_string()).collect();
        if split_did.len() != 2 {
            return Err(ClientError::DelegatedConf(
                "Delegate DID was not in expected format".to_string(),
            ));
        }

        let json_jwk = serde_json::to_string(&j).map_err(|e| {
            ClientError::DelegatedConf(format!("Could not serailize JWK from did resolver: {}", e))
        })?;

        Ok(Ed25519Jwk::new(&split_did[0], &json_jwk, &split_did[1])
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?)
    }

    pub async fn delegated_attestation(
        &self,
        statement: AttestationStatement,
    ) -> Result<DelegatedAttestationProof, ClientError> {
        // Check the ReCap is still valid.
        let p = self.is_valid().await?;
        // Check the JWK matches the ReCap
        let key = self.is_jwk_valid().await?;

        // Check that this is an approved opperation.
        let (t, _) = statement
            .to_statement()
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?;

        if !p.types.contains(&t) {
            return Err(ClientError::DelegatedConf(format!(
                "ReCap does not include capability: {}",
                to_action(&t)
            )));
        }

        let m = Message::from_str(&self.siwe_recap_message).map_err(|e| {
            ClientError::DelegatedConf(format!("Failed to parse ReCap into Message: {}", e))
        })?;

        // Check that the subject of the attestation matches the ReCap
        let expected_addr = eip55(&m.address);
        let expected_chain_id = format!("{}", m.chain_id);
        match statement.subject() {
            Subjects::Pkh(Pkh::Eip155(s)) => {
                if s.address != expected_addr {
                    return Err(ClientError::DelegatedConf(format!(
                        "Expected subject address {} got {}",
                        expected_addr, s.address
                    )));
                }

                if s.chain_id != expected_chain_id {
                    return Err(ClientError::DelegatedConf(format!(
                        "Expected chain id {} got {}",
                        expected_chain_id, s.chain_id
                    )));
                }
            }
            _ => {
                // TODO: Update if other keys are supported
                return Err(ClientError::DelegatedConf(
                    "Expected statement to have did:pkh:eip155, no other keys currently supported"
                        .to_string(),
                ));
            }
        };

        let plain_text = statement
            .generate_statement()
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?;

        let sig = key
            .sign(&plain_text)
            .await
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?;

        Ok(DelegatedAttestationProof {
            attestation: statement,
            attestation_signature: sig,
            service_key: self.service_key.clone(),
            siwe_message: self.siwe_recap_message.clone(),
            siwe_signature: self.siwe_recap_signature.clone(),
        })
    }
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

    // TODO: This will likely need more granular controls for integration with SSX.
    // TODO: This could possibly be a stand-alone fn since it has no ref to self
    pub async fn siwe_message(
        session_config: SessionConfig,
        service_key: &str,
        delegated_capabilities: &Vec<AttestationTypes>,
    ) -> Result<DelegatedAttestationPreConfig, ClientError> {
        let mut session_config = session_config;
        let s = session_config
            .generate_message(service_key, delegated_capabilities)
            .await
            .map_err(|e| ClientError::Capability(e.to_string()))?;

        Ok(DelegatedAttestationPreConfig {
            service_key: service_key.to_string(),
            session_config,
            siwe_recap_message: s,
        })
    }

    // TODO: Something better with T?
    // TODO: This could possibly be a stand-alone fn since it has no ref to self
    pub fn attestation_types_to_actions<T>(
        delegated_capabilities: &Vec<AttestationTypes>,
    ) -> Vec<(String, Vec<BTreeMap<String, T>>)> {
        delegated_capabilities
            .iter()
            .map(|c| (to_action(c), Vec::<BTreeMap<String, T>>::new()))
            .collect()
    }

    // TODO: Support LD credentials?
    pub async fn delegated_attestation_jwt(
        &self,
        delegated_attestation_config: DelegatedAttestationConfig,
        statement: AttestationStatement,
    ) -> Result<JWTWrapper, ClientError> {
        let proof = delegated_attestation_config
            .delegated_attestation(statement)
            .await?;

        // Check that the issuer is what was listed in the ReCap.
        // NOTE: From here down would change if doing a LD credential instead.
        let vc = self
            .witness_jwt(Proofs::DelegatedAttestation(proof))
            .await?;

        let v: Vec<String> = vc.jwt.split(".").map(|s| s.to_string()).collect();
        if v.len() < 2 {
            return Err(ClientError::DelegatedConf(
                "JWT was not in expected format".to_string(),
            ));
        }

        let s = general_purpose::STANDARD_NO_PAD
            .decode(v[1].clone())
            .map_err(|e| ClientError::DelegatedConf(e.to_string()))?;
        let s = String::from_utf8(s).map_err(|e| ClientError::DelegatedConf(e.to_string()))?;
        let w: JWTDecodeWrapper =
            serde_json::from_str(&s).map_err(|e| ClientError::DelegatedConf(e.to_string()))?;

        let did = delegated_attestation_config
            .service_key
            // TODO: Use a const or configurable prefix?
            .strip_prefix("rebase:")
            .ok_or(ClientError::DelegatedConf(
                "Failed to strip prefix from service_key".to_string(),
            ))?
            .to_string();

        if did != w.iss {
            return Err(ClientError::DelegatedConf(format!(
                "Expected credential with issuer: {} but got: {}",
                did, w.iss
            )));
        }

        // Make sure that the given issuer is the signer.
        self.verify(VCWrapper::Jwt(vc.clone())).await?;

        Ok(vc)
    }
}

#[derive(Deserialize, Serialize)]
struct JWTDecodeWrapper {
    iss: String,
}
