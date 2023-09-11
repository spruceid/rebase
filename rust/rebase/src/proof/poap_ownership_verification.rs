use crate::{
    content::poap_ownership_verification::PoapOwnershipVerificationContent as Ctnt,
    statement::poap_ownership_verification::PoapOwnershipVerificationStatement as Stmt,
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
pub struct PoapOwnershipVerificationProof {
    pub signature: String,
    pub statement: Stmt,
}

impl Statement for PoapOwnershipVerificationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for PoapOwnershipVerificationProof {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            event_id: format!("{}", self.statement.event_id.clone()),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
