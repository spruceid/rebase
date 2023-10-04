use crate::{
    content::same_controller_assertion::SameControllerAssertionContent as Ctnt,
    statement::same_controller_assertion::SameControllerAssertionStatement as Stmt,
    types::{
        defs::{Proof, Statement},
        error::{ProofError, StatementError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SameControllerAssertionProof {
    pub statement: Stmt,
    pub signature1: String,
    pub signature2: String,
}

impl Statement for SameControllerAssertionProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for SameControllerAssertionProof {
    fn to_content(&self, _statement: &str, _signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            id1: self.statement.id1.clone(),
            id2: self.statement.id2.clone(),
            statement: self.generate_statement()?,
            signature1: self.signature1.clone(),
            signature2: self.signature2.clone(),
        })
    }
}
