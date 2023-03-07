use crate::{
    content::reddit_verification::RedditVerification as Ctnt,
    statement::reddit_verification::RedditVerification as Stmt,
    types::{defs::Proof, error::ProofError},
};

impl Proof<Ctnt> for Stmt {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            handle: self.handle.clone(),
            subject: self.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
