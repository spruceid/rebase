use crate::{
    content::attestation::book_review_attestation::BookReviewAttestationContent,
    statement::attestation::book_review_attestation::BookReviewAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        enums::attestation::AttestationFormat,
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct BookReviewAttestationProof {
    pub statement: BookReviewAttestationStatement,
    pub signature: String,
}

impl Statement for BookReviewAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<BookReviewAttestationContent> for BookReviewAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<BookReviewAttestationContent, ProofError> {
        Ok(BookReviewAttestationContent {
            attestation_format: AttestationFormat::Attestation,
            id: self.statement.subject.did()?,
            link: self.statement.link.clone(),
            rating: self.statement.rating,
            review: self.statement.review.clone(),
            signature: self.signature.clone(),
            title: self.statement.title.clone(),
        })
    }
}
