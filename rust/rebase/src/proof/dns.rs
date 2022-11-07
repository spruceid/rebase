use crate::{
    content::dns::Dns as Ctnt, statement::dns::Dns as Stmt, types::error::ProofError,
    types::types::Proof,
};

impl Proof<Ctnt> for Stmt {
    fn to_content(&self, _statement: &str, _signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            domain: self.domain.clone(),
            subject: self.subject.clone(),
        })
    }
}
