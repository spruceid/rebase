use crate::{
    content::nft_ownership_verification::NftOwnershipVerificationContent as Ctnt,
    statement::nft_ownership_verification::NftOwnershipVerificationStatement as Stmt,
    types::{
        defs::{Proof, Statement},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct NftOwnershipVerificationProof {
    pub signature: String,
    pub statement: Stmt,
}

impl Statement for NftOwnershipVerificationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for NftOwnershipVerificationProof {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            contract_address: self.statement.contract_address.clone(),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
