use crate::{
    content::dns::Dns as Ctnt, statement::dns::Dns as Stmt, types::defs::Proof,
    types::error::ProofError,
};

impl Proof<Ctnt> for Stmt {
    fn to_content(&self, _statement: &str, _signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            domain: self.domain.clone(),
            subject: self.subject.clone(),
        })
    }
}
