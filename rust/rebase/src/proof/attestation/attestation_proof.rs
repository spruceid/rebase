use crate::{
    content::attestation::attestation_content::*,
    proof::attestation::{
        basic_image_attestation::*, basic_post_attestation::*, basic_profile_attestation::*,
        basic_tag_attestation::*, book_review_attestation::*, dapp_preferences_attestation::*,
        follow_attestation::*, like_attestation::*, progress_book_link_attestation::*,
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

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub enum AttestationProof {
    BasicImageAttestation(BasicImageAttestationProof),
    BasicPostAttestation(BasicPostAttestationProof),
    BasicProfileAttestation(BasicProfileAttestationProof),
    BasicTagAttestation(BasicTagAttestationProof),
    BookReviewAttestation(BookReviewAttestationProof),
    DappPreferencesAttestation(DappPreferencesAttestationProof),
    FollowAttestation(FollowAttestationProof),
    LikeAttestation(LikeAttestationProof),
    ProgressBookLinkAttestation(ProgressBookLinkAttestationProof),
}

impl AttestationProof {
    pub fn signature(&self) -> String {
        match self {
            AttestationProof::BasicImageAttestation(x) => x.signature.clone(),
            AttestationProof::BasicPostAttestation(x) => x.signature.clone(),
            AttestationProof::BasicProfileAttestation(x) => x.signature.clone(),
            AttestationProof::BasicTagAttestation(x) => x.signature.clone(),
            AttestationProof::BookReviewAttestation(x) => x.signature.clone(),
            AttestationProof::DappPreferencesAttestation(x) => x.signature.clone(),
            AttestationProof::FollowAttestation(x) => x.signature.clone(),
            AttestationProof::LikeAttestation(x) => x.signature.clone(),
            AttestationProof::ProgressBookLinkAttestation(x) => x.signature.clone(),
        }
    }

    pub fn subject(&self) -> Subjects {
        match self {
            AttestationProof::BasicImageAttestation(x) => x.statement.subject.clone(),
            AttestationProof::BasicPostAttestation(x) => x.statement.subject.clone(),
            AttestationProof::BasicProfileAttestation(x) => x.statement.subject.clone(),
            AttestationProof::BasicTagAttestation(x) => x.statement.subject.clone(),
            AttestationProof::BookReviewAttestation(x) => x.statement.subject.clone(),
            AttestationProof::DappPreferencesAttestation(x) => x.statement.subject.clone(),
            AttestationProof::FollowAttestation(x) => x.statement.subject.clone(),
            AttestationProof::LikeAttestation(x) => x.statement.subject.clone(),
            AttestationProof::ProgressBookLinkAttestation(x) => x.statement.subject.clone(),
        }
    }
}

impl Statement for AttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match self {
            AttestationProof::BasicImageAttestation(x) => x.generate_statement(),
            AttestationProof::BasicPostAttestation(x) => x.generate_statement(),
            AttestationProof::BasicProfileAttestation(x) => x.generate_statement(),
            AttestationProof::BasicTagAttestation(x) => x.generate_statement(),
            AttestationProof::BookReviewAttestation(x) => x.generate_statement(),
            AttestationProof::DappPreferencesAttestation(x) => x.generate_statement(),
            AttestationProof::FollowAttestation(x) => x.generate_statement(),
            AttestationProof::LikeAttestation(x) => x.generate_statement(),
            AttestationProof::ProgressBookLinkAttestation(x) => x.generate_statement(),
        }
    }
}

impl Proof<AttestationContent> for AttestationProof {
    fn to_content(
        &self,
        statement: &str,
        signature: &str,
    ) -> Result<AttestationContent, ProofError> {
        match &self {
            AttestationProof::BasicImageAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::BasicImageAttestation(c))
            }
            AttestationProof::BasicPostAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::BasicPostAttestation(c))
            }
            AttestationProof::BasicProfileAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::BasicProfileAttestation(c))
            }
            AttestationProof::BasicTagAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::BasicTagAttestation(c))
            }
            AttestationProof::BookReviewAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::BookReviewAttestation(c))
            }
            AttestationProof::DappPreferencesAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::DappPreferencesAttestation(c))
            }
            AttestationProof::FollowAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::FollowAttestation(c))
            }
            AttestationProof::LikeAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::LikeAttestation(c))
            }
            AttestationProof::ProgressBookLinkAttestation(x) => {
                let c = x.to_content(statement, signature)?;
                Ok(AttestationContent::ProgressBookLinkAttestation(c))
            }
        }
    }
}
