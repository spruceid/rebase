use crate::{
    content::witnessed_self_issued::content::*,
    proof::witnessed_self_issued::{
        witnessed_basic_image::*, witnessed_basic_post::*, witnessed_basic_profile::*,
        witnessed_basic_tag::*, witnessed_book_review::*, witnessed_dapp_preferences::*,
        witnessed_follow::*, witnessed_like::*, witnessed_progress_book_link::*,
    },
    types::{
        defs::{Proof, Statement},
        enums::subject::Subjects,
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub enum WitnessedSelfIssuedProof {
    WitnessedBasicImage(WitnessedBasicImageProof),
    WitnessedBasicPost(WitnessedBasicPostProof),
    WitnessedBasicProfile(WitnessedBasicProfileProof),
    WitnessedBasicTag(WitnessedBasicTagProof),
    WitnessedBookReview(WitnessedBookReviewProof),
    WitnessedDappPreferences(WitnessedDappPreferencesProof),
    WitnessedFollow(WitnessedFollowProof),
    WitnessedLike(WitnessedLikeProof),
    WitnessedProgressBookLink(WitnessedProgressBookLinkProof),
}

impl WitnessedSelfIssuedProof {
    pub fn signature(&self) -> String {
        match self {
            WitnessedSelfIssuedProof::WitnessedBasicImage(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedBasicPost(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedBasicProfile(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedBasicTag(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedBookReview(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedDappPreferences(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedFollow(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedLike(x) => x.signature.clone(),
            WitnessedSelfIssuedProof::WitnessedProgressBookLink(x) => x.signature.clone(),
        }
    }

    pub fn subject(&self) -> Subjects {
        match self {
            WitnessedSelfIssuedProof::WitnessedBasicImage(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedBasicPost(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedBasicProfile(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedBasicTag(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedBookReview(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedDappPreferences(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedFollow(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedLike(x) => x.statement.subject.clone(),
            WitnessedSelfIssuedProof::WitnessedProgressBookLink(x) => x.statement.subject.clone(),
        }
    }
}

impl Statement for WitnessedSelfIssuedProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match self {
            WitnessedSelfIssuedProof::WitnessedBasicImage(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedBasicPost(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedBasicProfile(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedBasicTag(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedBookReview(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedDappPreferences(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedFollow(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedLike(x) => x.generate_statement(),
            WitnessedSelfIssuedProof::WitnessedProgressBookLink(x) => x.generate_statement(),
        }
    }
}

impl Proof<WitnessedSelfIssuedContent> for WitnessedSelfIssuedProof {
    fn to_content(
        &self,
        statement: &str,
        signature: &str,
    ) -> Result<WitnessedSelfIssuedContent, ProofError> {
        match &self {
            WitnessedSelfIssuedProof::WitnessedBasicImage(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedBasicImage(c))
            }
            WitnessedSelfIssuedProof::WitnessedBasicPost(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedBasicPost(c))
            }
            WitnessedSelfIssuedProof::WitnessedBasicProfile(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedBasicProfile(c))
            }
            WitnessedSelfIssuedProof::WitnessedBasicTag(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedBasicTag(c))
            }
            WitnessedSelfIssuedProof::WitnessedBookReview(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedBookReview(c))
            }
            WitnessedSelfIssuedProof::WitnessedDappPreferences(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedDappPreferences(c))
            }
            WitnessedSelfIssuedProof::WitnessedFollow(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedFollow(c))
            }
            WitnessedSelfIssuedProof::WitnessedLike(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedLike(c))
            }
            WitnessedSelfIssuedProof::WitnessedProgressBookLink(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(WitnessedSelfIssuedContent::WitnessedProgressBookLink(c))
            }
        }
    }
}
