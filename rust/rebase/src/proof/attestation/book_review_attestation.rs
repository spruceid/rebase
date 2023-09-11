use crate::{
    content::attestation::book_review_attestation::BookReviewAttestationContent,
    statement::attestation::book_review_attestation::BookReviewAttestationStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
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
    ) -> Result<BookReviewAttestationContent, crate::types::error::ProofError> {
        Ok(BookReviewAttestationContent {
            id: self.statement.subject.did()?,
            link: self.statement.link.clone(),
            rating: self.statement.rating,
            review: self.statement.review.clone(),
            signature: self.signature.clone(),
            title: self.statement.title.clone(),
        })
    }
}
