use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{Signer, SignerError, SignerType};
use crate::witness::signer_type::SignerTypes;
use async_trait::async_trait;
use ssi::vc::Credential;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WitnessError {
    #[error("failed to lookup claim: {0}")]
    BadLookup(String),
    #[error("no id in given signer type")]
    NoId,
    #[error("failed to parse lookup: {0}")]
    ParseError(String),
    #[error("statement mismatch, expected: '{expected:?}' got: '{got:?}'")]
    StatementMismatch { expected: String, got: String },
    #[error("{0}")]
    SchemaError(#[from] SchemaError),
    #[error("{0}")]
    SignerError(#[from] SignerError),
}

#[async_trait(?Send)]
pub trait Proof
where
    Self: Sized,
{
    // TODO: Allow parse flow to be over-ridden?

    // From the proof structure, create an accurate statement for signing.
    fn generate_statement(&self) -> Result<String, WitnessError>;
    // the delimitor used to split between the statement and signature in post.
    fn delimitor(&self) -> String;
    // From the proof structure. generate a Signer type for validation
    fn signer_type(&self) -> Result<SignerTypes, SignerError>;

    // From the proof structure and any signer, create a valid attestation post.
    async fn generate_post<T: SignerType>(
        &self,
        signer: &dyn Signer<T>,
    ) -> Result<String, WitnessError> {
        let statement = self.generate_statement()?;
        let signature = signer.sign(&statement).await?;
        Ok(format!("{}{}{}", statement, self.delimitor(), signature))
    }

    // Parses post into statement and signature
    // And makes sure the signature and statement match the proof.
    async fn parse_post(&self, post: &str) -> Result<(String, String), WitnessError> {
        let d = self.delimitor();
        let v: Vec<&str> = post.split(&d).collect();
        if v.len() != 2 {
            return Err(WitnessError::ParseError(format!("incorrect post format")));
        };

        let (statement, signature) = (v[0].to_owned(), v[1].to_owned());
        let check_statement = self.generate_statement()?;
        if check_statement != statement {
            return Err(WitnessError::StatementMismatch {
                expected: check_statement,
                got: statement,
            });
        };

        self.signer_type()?
            .valid_signature(&statement, &signature)
            .await?;
        Ok((statement, signature))
    }
}

#[async_trait(?Send)]
pub trait Generator<P: Proof, S: SchemaType> {
    // From the proof structure, look up the statement and signature.
    async fn locate_post(&self, proof: &P) -> Result<String, WitnessError>;

    // From the proof structure, create a schema structure without any checks.
    fn _unchecked_to_schema(
        &self,
        proof: &P,
        statement: &str,
        signature: &str,
    ) -> Result<S, WitnessError>;

    // From the proof structure, create a schema.
    async fn schema(&self, proof: &P) -> Result<S, WitnessError> {
        let post = self.locate_post(proof).await?;
        let (statement, signature) = proof.parse_post(&post).await?;
        Ok(self._unchecked_to_schema(proof, &statement, &signature)?)
    }

    // From the proof structure, create a credential.
    async fn credential<T: SignerType>(
        &self,
        proof: &P,
        signer: &dyn Signer<T>,
    ) -> Result<Credential, WitnessError> {
        Ok(self.schema(proof).await?.credential(signer).await?)
    }
}
