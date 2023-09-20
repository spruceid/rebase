use crate::subject::{ed25519::Ed25519Jwk as Ed25519, ethereum::Eip155, solana::Solana};
use crate::types::{defs::Subject, error::SubjectError};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Subjects {
    #[serde(rename = "pkh")]
    Pkh(Pkh),
    #[serde(rename = "web")]
    Web(Web),
    #[serde(rename = "key")]
    Key(Key),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Key {
    #[serde(rename = "ed25519")]
    Ed25519(Ed25519),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Pkh {
    #[serde(rename = "eip155")]
    Eip155(Eip155),
    #[serde(rename = "solana")]
    Solana(Solana),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Web {
    #[serde(rename = "ed25519")]
    Ed25519(Ed25519),
}

#[async_trait(?Send)]
impl Subject for Subjects {
    fn did(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.did(),
            Subjects::Pkh(Pkh::Solana(x)) => x.did(),
            Subjects::Web(Web::Ed25519(x)) => x.did(),
            Subjects::Key(Key::Ed25519(x)) => x.did(),
        }
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.display_id(),
            Subjects::Pkh(Pkh::Solana(x)) => x.display_id(),
            Subjects::Web(Web::Ed25519(x)) => x.display_id(),
            Subjects::Key(Key::Ed25519(x)) => x.display_id(),
        }
    }

    fn verification_method(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.verification_method(),
            Subjects::Pkh(Pkh::Solana(x)) => x.verification_method(),
            Subjects::Web(Web::Ed25519(x)) => x.verification_method(),
            Subjects::Key(Key::Ed25519(x)) => x.verification_method(),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(x)) => x.valid_signature(statement, signature).await,
            Subjects::Pkh(Pkh::Solana(x)) => x.valid_signature(statement, signature).await,
            Subjects::Web(Web::Ed25519(x)) => x.valid_signature(statement, signature).await,
            Subjects::Key(Key::Ed25519(x)) => x.valid_signature(statement, signature).await,
        }
    }
}

// NOTE: This being here allows all internationalization to occur in the Statements -> Subects,
// dependency rather than spread between Statement and Subject.
impl Subjects {
    pub fn statement_title(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::Pkh(Pkh::Eip155(_)) => Ok("Ethereum Address".to_string()),
            Subjects::Pkh(Pkh::Solana(_)) => Ok("Solana Address".to_string()),
            Subjects::Web(Web::Ed25519(_)) => Ok("Ed25519 Web Key".to_string()),
            Subjects::Key(Key::Ed25519(_)) => Ok("Ed25519 DID Key".to_string()),
        }
    }
}
