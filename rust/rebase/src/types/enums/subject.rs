use crate::subject::{
    did_subject::DidSubject, ed25519::DidWeb as Ed25519, ethereum::Eip155, solana::Solana,
};
use crate::types::{defs::Subject, error::SubjectError};

use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// TODO: Delete ALL of this and ONLY use DidSubject and DidIssuer!!!
// TODO: Deprecate and move all demos off of this scheme!!!
#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "subject", untagged)]
#[ts(export, rename = "Subjects")]
pub enum Subjects {
    BackwardsCompat(InnerSubjects),
    Subject(DidSubject),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
pub enum InnerSubjects {
    // TODO: DELETE!
    #[serde(rename = "pkh")]
    Pkh(Pkh),
    // TODO: DELETE!
    #[serde(rename = "web")]
    Web(Web),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "pkh")]
#[ts(export, rename = "Pkh")]
pub enum Pkh {
    #[serde(rename = "eip155")]
    Eip155(Eip155),
    #[serde(rename = "solana")]
    Solana(Solana),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "web")]
#[ts(export, rename = "Web")]
pub enum Web {
    #[serde(rename = "ed25519")]
    Ed25519(Ed25519),
}

#[async_trait(?Send)]
impl Subject for Subjects {
    fn did(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Eip155(x))) => x.did(),
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Solana(x))) => x.did(),
            Subjects::BackwardsCompat(InnerSubjects::Web(Web::Ed25519(x))) => x.did(),
            Subjects::Subject(x) => x.did(),
        }
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Eip155(x))) => x.display_id(),
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Solana(x))) => x.display_id(),
            Subjects::BackwardsCompat(InnerSubjects::Web(Web::Ed25519(x))) => x.display_id(),
            Subjects::Subject(x) => x.display_id(),
        }
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        match &self {
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Eip155(x))) => {
                x.valid_signature(statement, signature).await
            }
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Solana(x))) => {
                x.valid_signature(statement, signature).await
            }
            Subjects::BackwardsCompat(InnerSubjects::Web(Web::Ed25519(x))) => {
                x.valid_signature(statement, signature).await
            }
            Subjects::Subject(x) => x.valid_signature(statement, signature).await,
        }
    }
}

// NOTE: This being here allows all internationalization to occur in the Statements -> Subects,
// dependency rather than spread between Statement and Subject.
impl Subjects {
    pub fn statement_title(&self) -> Result<String, SubjectError> {
        match &self {
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Eip155(_))) => {
                Ok("Ethereum Address".to_string())
            }
            Subjects::BackwardsCompat(InnerSubjects::Pkh(Pkh::Solana(_))) => {
                Ok("Solana Address".to_string())
            }
            Subjects::BackwardsCompat(InnerSubjects::Web(Web::Ed25519(_))) => {
                Ok("Ed25519 Web Key".to_string())
            }
            Subjects::Subject(x) => {
                // TODO: Make this comprehensive!
                if x.did.starts_with("did:web") {
                    // TODO: RESTORE THIS AFTER UPDATING TESTS!
                    // return Ok("DID Web Key".to_string());
                    return Ok("Ed25519 Web Key".to_string());
                }

                if x.did.starts_with("did:pkh:eip155") {
                    return Ok("Ethereum Address".to_string());
                }

                if x.did.starts_with("did:pkh:solana") {
                    return Ok("Solana Address".to_string());
                }

                Ok("DID ID".to_string())
            }
        }
    }
}
