use crate::{
    content::witnessed_basic_profile::WitnessedBasicProfileContent,
    proof::witnessed_basic_profile::WitnessedBasicProfileProof,
    statement::witnessed_basic_profile::WitnessedBasicProfileStatement,
    types::{
        defs::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
        error::FlowError,
    },
};
use async_trait::async_trait;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

// TOOD: Change to generalize over all witnessed self-issued.
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessedBasicProfileFlow {}

#[async_trait(?Send)]
impl Flow<WitnessedBasicProfileContent, WitnessedBasicProfileStatement, WitnessedBasicProfileProof>
    for WitnessedBasicProfileFlow
{
    fn instructions(&self) -> Result<Instructions, FlowError> {
        // NOTE: These instructions are for all witnessed flows.
        Ok(Instructions {
            statement: "Fill out the presented form to create content in the form of a credential."
                .to_string(),
            statement_schema: schema_for!(WitnessedBasicProfileStatement),
            signature: "Sign a plain-text version of the content created in the previous step."
                .to_string(),
            witness: "Present the signature and the content object to the witness to have it transformed into a credential.".to_string(),
            witness_schema: schema_for!(WitnessedBasicProfileProof)
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &WitnessedBasicProfileStatement,
        _issuer: &I,
    ) -> Result<FlowResponse, FlowError> {
        Ok(FlowResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &WitnessedBasicProfileProof,
        _issuer: &I,
    ) -> Result<WitnessedBasicProfileContent, FlowError> {
        let stmt = proof.generate_statement()?;
        proof
            .statement
            .subject
            .valid_signature(&stmt, &proof.signature)
            .await?;
        Ok(proof.to_content(&stmt, &proof.signature)?)
    }
}
