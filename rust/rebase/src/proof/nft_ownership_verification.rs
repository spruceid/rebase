use crate::{
    content::nft_ownership_verification::NftOwnershipVerification as Ctnt,
    statement::nft_ownership_verification::NftOwnershipVerification as Stmt,
    types::{
        defs::{Proof, Statement},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "proof")]
pub struct NftOwnershipVerification {
    pub signature: String,
    pub statement: Stmt,
}

impl Statement for NftOwnershipVerification {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for NftOwnershipVerification {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            contract_address: self.statement.contract_address.clone(),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
