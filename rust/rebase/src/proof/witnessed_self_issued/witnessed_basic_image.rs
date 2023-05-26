use crate::{
    content::witnessed_self_issued::witnessed_basic_image::WitnessedBasicImageContent,
    statement::witnessed_self_issued::witnessed_basic_image::WitnessedBasicImageStatement,
    types::{
        defs::{Proof, Statement, Subject},
        error::StatementError,
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicImageProof {
    pub statement: WitnessedBasicImageStatement,
    pub signature: String,
}

impl Statement for WitnessedBasicImageProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedBasicImageContent> for WitnessedBasicImageProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedBasicImageContent, crate::types::error::ProofError> {
        Ok(WitnessedBasicImageContent {
            id: self.statement.subject.did()?,
            src: self.statement.src.clone(),
            signature: self.signature.clone(),
        })
    }
}
