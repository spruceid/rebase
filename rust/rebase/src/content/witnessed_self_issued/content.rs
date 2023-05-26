use crate::{
    content::witnessed_self_issued::{
        witnessed_basic_image::*, witnessed_basic_post::*, witnessed_basic_profile::*,
        witnessed_basic_tag::*, witnessed_book_review::*, witnessed_dapp_preferences::*,
        witnessed_follow::*, witnessed_like::*, witnessed_progress_book_link::*,
    },
    types::{defs::Content, error::ContentError},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use ts_rs::TS;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub enum WitnessedSelfIssuedContent {
    WitnessedBasicImage(WitnessedBasicImageContent),
    WitnessedBasicPost(WitnessedBasicPostContent),
    WitnessedBasicProfile(WitnessedBasicProfileContent),
    WitnessedBasicTag(WitnessedBasicTagContent),
    WitnessedBookReview(WitnessedBookReviewContent),
    WitnessedDappPreferences(WitnessedDappPreferencesContent),
    WitnessedFollow(WitnessedFollowContent),
    WitnessedLike(WitnessedLikeContent),
    WitnessedProgressBookLink(WitnessedProgressBookLinkContent),
}

impl Content for WitnessedSelfIssuedContent {
    fn context(&self) -> Result<Value, ContentError> {
        match self {
            WitnessedSelfIssuedContent::WitnessedBasicImage(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedBasicPost(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedBasicProfile(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedBasicTag(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedBookReview(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedDappPreferences(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedFollow(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedLike(x) => x.context(),
            WitnessedSelfIssuedContent::WitnessedProgressBookLink(x) => x.context(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            WitnessedSelfIssuedContent::WitnessedBasicImage(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedBasicPost(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedBasicProfile(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedBasicTag(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedBookReview(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedDappPreferences(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedFollow(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedLike(x) => x.types(),
            WitnessedSelfIssuedContent::WitnessedProgressBookLink(x) => x.types(),
        }
    }

    fn subject(&self) -> Result<Value, ContentError> {
        match self {
            WitnessedSelfIssuedContent::WitnessedBasicImage(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedBasicPost(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedBasicProfile(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedBasicTag(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedBookReview(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedDappPreferences(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedFollow(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedLike(x) => x.subject(),
            WitnessedSelfIssuedContent::WitnessedProgressBookLink(x) => x.subject(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            WitnessedSelfIssuedContent::WitnessedBasicImage(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedBasicPost(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedBasicProfile(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedBasicTag(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedBookReview(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedDappPreferences(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedFollow(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedLike(x) => x.evidence(),
            WitnessedSelfIssuedContent::WitnessedProgressBookLink(x) => x.evidence(),
        }
    }
}
