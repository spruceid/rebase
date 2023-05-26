use crate::{
    content::witnessed_self_issued::witnessed_basic_profile::WitnessedBasicProfileContent,
    statement::witnessed_self_issued::witnessed_basic_profile::WitnessedBasicProfileStatement,
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
pub struct WitnessedBasicProfileProof {
    pub statement: WitnessedBasicProfileStatement,
    pub signature: String,
}

impl Statement for WitnessedBasicProfileProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<WitnessedBasicProfileContent> for WitnessedBasicProfileProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<WitnessedBasicProfileContent, crate::types::error::ProofError> {
        Ok(WitnessedBasicProfileContent {
            description: self.statement.description.clone(),
            id: self.statement.subject.did()?,
            image: self.statement.image.clone(),
            username: self.statement.username.clone(),
            website: self.statement.website.clone(),
            signature: self.signature.clone(),
        })
    }
}
