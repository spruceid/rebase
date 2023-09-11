use crate::{
    content::attestation::{
        basic_image_attestation::*, basic_post_attestation::*, basic_profile_attestation::*,
        basic_tag_attestation::*, book_review_attestation::*, dapp_preferences_attestation::*,
        follow_attestation::*, like_attestation::*, progress_book_link_attestation::*,
    },
    types::{defs::Content, error::ContentError},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum AttestationContent {
    BasicImageAttestation(BasicImageAttestationContent),
    BasicPostAttestation(BasicPostAttestationContent),
    BasicProfileAttestation(BasicProfileAttestationContent),
    BasicTagAttestation(BasicTagAttestationContent),
    BookReviewAttestation(BookReviewAttestationContent),
    DappPreferencesAttestation(DappPreferencesAttestationContent),
    FollowAttestation(FollowAttestationContent),
    LikeAttestation(LikeAttestationContent),
    ProgressBookLinkAttestation(ProgressBookLinkAttestationContent),
}

impl Content for AttestationContent {
    fn context(&self) -> Result<Value, ContentError> {
        match self {
            AttestationContent::BasicImageAttestation(x) => x.context(),
            AttestationContent::BasicPostAttestation(x) => x.context(),
            AttestationContent::BasicProfileAttestation(x) => x.context(),
            AttestationContent::BasicTagAttestation(x) => x.context(),
            AttestationContent::BookReviewAttestation(x) => x.context(),
            AttestationContent::DappPreferencesAttestation(x) => x.context(),
            AttestationContent::FollowAttestation(x) => x.context(),
            AttestationContent::LikeAttestation(x) => x.context(),
            AttestationContent::ProgressBookLinkAttestation(x) => x.context(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            AttestationContent::BasicImageAttestation(x) => x.types(),
            AttestationContent::BasicPostAttestation(x) => x.types(),
            AttestationContent::BasicProfileAttestation(x) => x.types(),
            AttestationContent::BasicTagAttestation(x) => x.types(),
            AttestationContent::BookReviewAttestation(x) => x.types(),
            AttestationContent::DappPreferencesAttestation(x) => x.types(),
            AttestationContent::FollowAttestation(x) => x.types(),
            AttestationContent::LikeAttestation(x) => x.types(),
            AttestationContent::ProgressBookLinkAttestation(x) => x.types(),
        }
    }

    fn subject(&self) -> Result<Value, ContentError> {
        match self {
            AttestationContent::BasicImageAttestation(x) => x.subject(),
            AttestationContent::BasicPostAttestation(x) => x.subject(),
            AttestationContent::BasicProfileAttestation(x) => x.subject(),
            AttestationContent::BasicTagAttestation(x) => x.subject(),
            AttestationContent::BookReviewAttestation(x) => x.subject(),
            AttestationContent::DappPreferencesAttestation(x) => x.subject(),
            AttestationContent::FollowAttestation(x) => x.subject(),
            AttestationContent::LikeAttestation(x) => x.subject(),
            AttestationContent::ProgressBookLinkAttestation(x) => x.subject(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            AttestationContent::BasicImageAttestation(x) => x.evidence(),
            AttestationContent::BasicPostAttestation(x) => x.evidence(),
            AttestationContent::BasicProfileAttestation(x) => x.evidence(),
            AttestationContent::BasicTagAttestation(x) => x.evidence(),
            AttestationContent::BookReviewAttestation(x) => x.evidence(),
            AttestationContent::DappPreferencesAttestation(x) => x.evidence(),
            AttestationContent::FollowAttestation(x) => x.evidence(),
            AttestationContent::LikeAttestation(x) => x.evidence(),
            AttestationContent::ProgressBookLinkAttestation(x) => x.evidence(),
        }
    }
}
