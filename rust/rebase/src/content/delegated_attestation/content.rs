use crate::{
    content::delegated_attestation::{
        delegated_basic_image_attestation::*, delegated_basic_post_attestation::*,
        delegated_basic_profile_attestation::*, delegated_basic_tag_attestation::*,
        delegated_book_review_attestation::*, delegated_dapp_preferences_attestation::*,
        delegated_follow_attestation::*, delegated_like_attestation::*,
        delegated_progress_book_link_attestation::*,
    },
    types::{defs::Content, error::ContentError},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum DelegatedAttestationContent {
    BasicImageAttestation(DelegatedBasicImageAttestationContent),
    BasicPostAttestation(DelegatedBasicPostAttestationContent),
    BasicProfileAttestation(DelegatedBasicProfileAttestationContent),
    BasicTagAttestation(DelegatedBasicTagAttestationContent),
    BookReviewAttestation(DelegatedBookReviewAttestationContent),
    DappPreferencesAttestation(DelegatedDappPreferencesAttestationContent),
    FollowAttestation(DelegatedFollowAttestationContent),
    LikeAttestation(DelegatedLikeAttestationContent),
    ProgressBookLinkAttestation(DelegatedProgressBookLinkAttestationContent),
}

impl Content for DelegatedAttestationContent {
    fn context(&self) -> Result<Value, ContentError> {
        match self {
            DelegatedAttestationContent::BasicImageAttestation(x) => x.context(),
            DelegatedAttestationContent::BasicPostAttestation(x) => x.context(),
            DelegatedAttestationContent::BasicProfileAttestation(x) => x.context(),
            DelegatedAttestationContent::BasicTagAttestation(x) => x.context(),
            DelegatedAttestationContent::BookReviewAttestation(x) => x.context(),
            DelegatedAttestationContent::DappPreferencesAttestation(x) => x.context(),
            DelegatedAttestationContent::FollowAttestation(x) => x.context(),
            DelegatedAttestationContent::LikeAttestation(x) => x.context(),
            DelegatedAttestationContent::ProgressBookLinkAttestation(x) => x.context(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            DelegatedAttestationContent::BasicImageAttestation(x) => x.types(),
            DelegatedAttestationContent::BasicPostAttestation(x) => x.types(),
            DelegatedAttestationContent::BasicProfileAttestation(x) => x.types(),
            DelegatedAttestationContent::BasicTagAttestation(x) => x.types(),
            DelegatedAttestationContent::BookReviewAttestation(x) => x.types(),
            DelegatedAttestationContent::DappPreferencesAttestation(x) => x.types(),
            DelegatedAttestationContent::FollowAttestation(x) => x.types(),
            DelegatedAttestationContent::LikeAttestation(x) => x.types(),
            DelegatedAttestationContent::ProgressBookLinkAttestation(x) => x.types(),
        }
    }

    fn subject(&self) -> Result<Value, ContentError> {
        match self {
            DelegatedAttestationContent::BasicImageAttestation(x) => x.subject(),
            DelegatedAttestationContent::BasicPostAttestation(x) => x.subject(),
            DelegatedAttestationContent::BasicProfileAttestation(x) => x.subject(),
            DelegatedAttestationContent::BasicTagAttestation(x) => x.subject(),
            DelegatedAttestationContent::BookReviewAttestation(x) => x.subject(),
            DelegatedAttestationContent::DappPreferencesAttestation(x) => x.subject(),
            DelegatedAttestationContent::FollowAttestation(x) => x.subject(),
            DelegatedAttestationContent::LikeAttestation(x) => x.subject(),
            DelegatedAttestationContent::ProgressBookLinkAttestation(x) => x.subject(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            DelegatedAttestationContent::BasicImageAttestation(x) => x.evidence(),
            DelegatedAttestationContent::BasicPostAttestation(x) => x.evidence(),
            DelegatedAttestationContent::BasicProfileAttestation(x) => x.evidence(),
            DelegatedAttestationContent::BasicTagAttestation(x) => x.evidence(),
            DelegatedAttestationContent::BookReviewAttestation(x) => x.evidence(),
            DelegatedAttestationContent::DappPreferencesAttestation(x) => x.evidence(),
            DelegatedAttestationContent::FollowAttestation(x) => x.evidence(),
            DelegatedAttestationContent::LikeAttestation(x) => x.evidence(),
            DelegatedAttestationContent::ProgressBookLinkAttestation(x) => x.evidence(),
        }
    }
}
