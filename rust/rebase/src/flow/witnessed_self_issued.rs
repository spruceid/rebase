use crate::{
    content::witnessed_self_issued::content::WitnessedSelfIssuedContent,
    proof::witnessed_self_issued::proof::WitnessedSelfIssuedProof,
    statement::witnessed_self_issued::statement::WitnessedSelfIssuedStatement,
    types::{
        defs::{Flow, FlowResponse, Instructions, Issuer, Proof, Statement, Subject},
        error::FlowError,
    },
};
use async_trait::async_trait;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessedSelfIssuedFlow {}

#[async_trait(?Send)]
impl Flow<WitnessedSelfIssuedContent, WitnessedSelfIssuedStatement, WitnessedSelfIssuedProof>
    for WitnessedSelfIssuedFlow
{
    fn instructions(&self) -> Result<Instructions, FlowError> {
        // NOTE: These instructions are for all witnessed flows.
        Ok(Instructions {
            statement: "Fill out the presented form to create content in the form of a credential."
                .to_string(),
            statement_schema: schema_for!(WitnessedSelfIssuedStatement),
            signature: "Sign a plain-text version of the content created in the previous step."
                .to_string(),
            witness: "Present the signature and the content object to the witness to have it transformed into a credential.".to_string(),
            witness_schema: schema_for!(WitnessedSelfIssuedProof)
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &WitnessedSelfIssuedStatement,
        _issuer: &I,
    ) -> Result<FlowResponse, FlowError> {
        Ok(FlowResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &WitnessedSelfIssuedProof,
        _issuer: &I,
    ) -> Result<WitnessedSelfIssuedContent, FlowError> {
        let stmt = proof.generate_statement()?;
        proof
            .subject()
            .valid_signature(&stmt, &proof.signature())
            .await?;
        Ok(proof.to_content(&stmt, &proof.signature())?)
    }
}
