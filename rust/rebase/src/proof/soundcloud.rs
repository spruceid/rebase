use crate::{
    content::soundcloud::SoundCloud as Ctnt,
    statement::soundcloud::SoundCloud as Stmt,
    types::{defs::Proof, error::ProofError},
};

impl Proof<Ctnt> for Stmt {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            permalink: self.permalink.clone(),
            subject: self.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
