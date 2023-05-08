use crate::{
    content::witnessed_self_issued::witnessed_book_review::WitnessedBookReviewContent,
    statement::witnessed_self_issued::witnessed_book_review::WitnessedBookReviewStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBookReviewProof {
    pub statement: WitnessedBookReviewStatement,
    pub signature: String,
}

impl Statement for WitnessedBookReviewProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedBookReviewContent> for WitnessedBookReviewProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedBookReviewContent, crate::types::error::ProofError> {
        Ok(WitnessedBookReviewContent {
            id: self.statement.subject.did()?,
            link: self.statement.link.clone(),
            rating: self.statement.rating,
            review: self.statement.review.clone(),
            signature: self.signature.clone(),
            title: self.statement.title.clone(),
        })
    }
}
