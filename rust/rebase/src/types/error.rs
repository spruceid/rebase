use serde_json::Error as SerializeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RebaseError {
    #[error("capability error: {0}")]
    CapabilityError(#[from] CapabilityError),
    #[error("content error: {0}")]
    Content(#[from] ContentError),
    #[error("issuer error: {0}")]
    Issuer(#[from] IssuerError),
    #[error("subject error: {0}")]
    Subject(#[from] SubjectError),
    #[error("statement error: {0}")]
    Statement(#[from] StatementError),
    #[error("proof error: {0}")]
    Proof(#[from] ProofError),
    #[error("flow error: {0}")]
    Flow(#[from] FlowError),
}

#[derive(Error, Debug)]
pub enum CapabilityError {
    #[error("recap error: {0}")]
    ReCapError(String),
}

#[derive(Error, Debug)]
pub enum SubjectError {
    #[error("failed to generate subject type: {0}")]
    SubjType(String),
    #[error("invalid signature: {0}")]
    Validation(String),
    #[error("failed to generate did string: {0}")]
    Did(String),
}

#[derive(Error, Debug)]
pub enum IssuerError {
    #[error("{0}")]
    Subject(#[from] SubjectError),
    #[error("failed to generate signature: {0}")]
    Sign(String),
    #[error("failed to sign verifiable credential: {0}")]
    Vc(String),
    #[error("failed to generate jwt: {0}")]
    Jwt(String),
    #[error("failed to generate proof: {0}")]
    Proof(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Error, Debug)]
pub enum ContentError {
    #[error("{0}")]
    Subject(#[from] SubjectError),
    #[error("{0}")]
    Issuer(#[from] IssuerError),
    #[error{"serialization error: {0}"}]
    Serialize(#[from] SerializeError),
    #[error{"invalid content: {0}"}]
    Invalid(String),
}

#[derive(Error, Debug)]
pub enum StatementError {
    #[error("failed to generate statement: {0}")]
    Statement(String),
    #[error("{0}")]
    Subject(#[from] SubjectError),
}

#[derive(Error, Debug)]
pub enum ProofError {
    #[error("failed to generate content: {0}")]
    ContentGeneration(String),
    #[error("{0}")]
    Statement(#[from] StatementError),
    #[error("{0}")]
    Subject(#[from] SubjectError),
}

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("failed in proof look up: {0}")]
    BadLookup(String),
    #[error("failed to validate proof: {0}")]
    Validation(String),
    #[error("{0}")]
    Content(#[from] ContentError),
    #[error("{0}")]
    Proof(#[from] ProofError),
    #[error("{0}")]
    Statement(#[from] StatementError),
    #[error("{0}")]
    Subject(#[from] SubjectError),
    #[error("{0}")]
    Issuer(#[from] IssuerError),
}
