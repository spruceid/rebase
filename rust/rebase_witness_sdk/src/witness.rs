pub use rebase::signer::signer::{Signer, SignerType};
pub use rebase::witness::email::EmailClient;
use rebase::witness::{
    generator::{Credential as VC, Opts as WOpts, WitnessGenerator as Generator},
    instructions::InstructionTypes,
    proof_type::ProofTypes,
    statement_type::StatementTypes,
    witness::Statement,
};

use thiserror::Error;

use serde::{Deserialize, Serialize};

pub type Credential = VC;
pub type WitnessGenerator = Generator;
pub type WitnessOpts = WOpts;

#[derive(Error, Debug)]
pub enum WitnessError {
    #[error("issue in instruction flow: {0}")]
    Instruction(String),
    #[error("issue in statement flow: {0}")]
    Statement(String),
    #[error("issue in witness flow: {0}")]
    Witness(String),
}

#[derive(Deserialize, Serialize)]
pub struct InstructionReq {
    #[serde(rename = "type")]
    pub instruction_type: InstructionTypes,
}

#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    pub opts: StatementTypes,
}

#[derive(Deserialize, Serialize)]
pub struct StatementRes {
    pub statement: String,
    pub delimitor: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: ProofTypes,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WitnessJWTRes {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessLDRes {
    pub credential: Credential,
}

pub fn instructions(req: InstructionReq) -> Result<serde_json::Value, WitnessError> {
    req.instruction_type
        .ui_hints()
        .map_err(|e| WitnessError::Instruction(e.to_string()))
}

pub async fn statement(
    req: StatementReq,
    generator: &WitnessGenerator,
) -> Result<StatementRes, WitnessError> {
    // Specical case email and other flows where the Witness makes the post.
    let d = req.opts.delimitor();
    let s = match req.opts {
        StatementTypes::Email(o) => match &generator.email {
            None => {
                return Err(WitnessError::Statement(
                    "No Email Generator Configurated".to_string(),
                ))
            }
            Some(email_gen) => email_gen.statement_and_send(&o).await.map_err(|e| {
                WitnessError::Statement(format!(
                    "Failed to send email after generating statement: {}",
                    e.to_string()
                ))
            })?,
        },
        _ => req
            .opts
            .generate_statement()
            .map_err(|e| WitnessError::Statement(e.to_string()))?,
    };

    let res = StatementRes {
        statement: s,
        delimitor: d,
    };

    Ok(res)
}

pub async fn witness_jwt<T: SignerType, S: Signer<T>>(
    witness_request: WitnessReq,
    generator: &WitnessGenerator,
    signer: &S,
) -> Result<WitnessJWTRes, WitnessError> {
    let jwt = generator
        .witness_jwt(&witness_request.proof, signer)
        .await
        .map_err(|e| WitnessError::Witness(e.to_string()))?;
    let res = WitnessJWTRes { jwt };

    Ok(res)
}

pub async fn witness_ld<T: SignerType, S: Signer<T>>(
    witness_request: WitnessReq,
    generator: &WitnessGenerator,
    signer: &S,
) -> Result<WitnessLDRes, WitnessError> {
    let credential = generator
        .witness_ld(&witness_request.proof, signer)
        .await
        .map_err(|e| WitnessError::Witness(e.to_string()))?;
    let res = WitnessLDRes { credential };

    Ok(res)
}
