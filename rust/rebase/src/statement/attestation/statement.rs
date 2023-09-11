use crate::{
    statement::attestation::{
        basic_image_attestation::*, basic_post_attestation::*, basic_profile_attestation::*,
        basic_tag_attestation::*, book_review_attestation::*, dapp_preferences_attestation::*,
        follow_attestation::*, like_attestation::*, progress_book_link_attestation::*,
    },
    types::{
        enums::{attestation::*, subject::Subjects},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum AttestationStatement {
    BasicImageAttestation(BasicImageAttestationStatement),
    BasicPostAttestation(BasicPostAttestationStatement),
    BasicProfileAttestation(BasicProfileAttestationStatement),
    BasicTagAttestation(BasicTagAttestationStatement),
    BookReviewAttestation(BookReviewAttestationStatement),
    DappPreferencesAttestation(DappPreferencesAttestationStatement),
    FollowAttestation(FollowAttestationStatement),
    LikeAttestation(LikeAttestationStatement),
    ProgressBookLinkAttestation(ProgressBookLinkAttestationStatement),
}

impl AttestationStatement {
    pub fn subject(&self) -> Subjects {
        match &self {
            AttestationStatement::BasicImageAttestation(x) => x.subject.clone(),
            AttestationStatement::BasicPostAttestation(x) => x.subject.clone(),
            AttestationStatement::BasicProfileAttestation(x) => x.subject.clone(),
            AttestationStatement::BasicTagAttestation(x) => x.subject.clone(),
            AttestationStatement::BookReviewAttestation(x) => x.subject.clone(),
            AttestationStatement::DappPreferencesAttestation(x) => x.subject.clone(),
            AttestationStatement::FollowAttestation(x) => x.subject.clone(),
            AttestationStatement::LikeAttestation(x) => x.subject.clone(),
            AttestationStatement::ProgressBookLinkAttestation(x) => x.subject.clone(),
        }
    }
}

impl Attestation for AttestationStatement {
    fn to_statement(&self) -> Result<(AttestationTypes, Map<String, Value>), StatementError> {
        match &self {
            AttestationStatement::BasicImageAttestation(x) => x.to_statement(),
            AttestationStatement::BasicPostAttestation(x) => x.to_statement(),
            AttestationStatement::BasicProfileAttestation(x) => x.to_statement(),
            AttestationStatement::BasicTagAttestation(x) => x.to_statement(),
            AttestationStatement::BookReviewAttestation(x) => x.to_statement(),
            AttestationStatement::DappPreferencesAttestation(x) => x.to_statement(),
            AttestationStatement::FollowAttestation(x) => x.to_statement(),
            AttestationStatement::LikeAttestation(x) => x.to_statement(),
            AttestationStatement::ProgressBookLinkAttestation(x) => x.to_statement(),
        }
    }
}
