use crate::{
    content::twitter_verification::TwitterVerificationContent as Ctnt,
    statement::twitter_verification::TwitterVerificationStatement as Stmt,
    types::{
        defs::{Proof, Statement},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct TwitterVerificationProof {
    pub statement: Stmt,
    pub tweet_url: String,
}

impl Statement for TwitterVerificationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for TwitterVerificationProof {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            handle: self.statement.handle.clone(),
            subject: self.statement.subject.clone(),
            tweet_url: self.tweet_url.to_owned(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
