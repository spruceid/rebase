use crate::{
    content::reddit::Reddit as Ctnt,
    statement::reddit::Reddit as Stmt,
    types::{error::ProofError, types::Proof},
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
