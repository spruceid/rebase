use crate::{
    statement::witnessed_self_issued::{
        witnessed_basic_image::*, witnessed_basic_post::*, witnessed_basic_profile::*,
        witnessed_basic_tag::*, witnessed_book_review::*, witnessed_dapp_preferences::*,
        witnessed_follow::*, witnessed_like::*, witnessed_progress_book_link::*,
    },
    types::{enums::witnessed_self_issued::*, error::StatementError},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub enum WitnessedSelfIssuedStatement {
    WitnessedBasicImage(WitnessedBasicImageStatement),
    WitnessedBasicPost(WitnessedBasicPostStatement),
    WitnessedBasicProfile(WitnessedBasicProfileStatement),
    WitnessedBasicTag(WitnessedBasicTagStatement),
    WitnessedBookReview(WitnessedBookReviewStatement),
    WitnessedDappPreferences(WitnessedDappPreferencesStatement),
    WitnessedFollow(WitnessedFollowStatement),
    WitnessedLike(WitnessedLikeStatement),
    WitnessedProgressBookLink(WitnessedProgressBookLinkStatement),
}

impl WitnesssedSelfIssued for WitnessedSelfIssuedStatement {
    fn to_statement(
        &self,
    ) -> Result<(WitnessedSelfIssuedTypes, Map<String, Value>), StatementError> {
        match &self {
            WitnessedSelfIssuedStatement::WitnessedBasicImage(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedBasicPost(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedBasicProfile(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedBasicTag(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedBookReview(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedDappPreferences(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedFollow(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedLike(x) => x.to_statement(),
            WitnessedSelfIssuedStatement::WitnessedProgressBookLink(x) => x.to_statement(),
        }
    }
}
