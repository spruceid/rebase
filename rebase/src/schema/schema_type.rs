// TODO: Make this less ugly.
use crate::signer::signer::{Signer, SignerError, SignerType};
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use serde_json::{json, Error as SeralizeError};
use ssi::{
    one_or_many::OneOrMany,
    vc::{Credential, Evidence},
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("{0}")]
    Serialize(#[from] SeralizeError),
    #[error("{0}")]
    Signer(#[from] SignerError),
}

#[async_trait(?Send)]
pub trait SchemaType {
    // Return the complete, signed credential
    async fn credential<T: SignerType>(&self, signer: &dyn Signer<T>) -> Result<Credential, SchemaError> {
        let did = signer.as_did().await?;

        let mut vc: Credential = serde_json::from_value(json!({
            "@context": self.context()?,
            "id": format!("urn:uuid:{}", Uuid::new_v4().to_string()),
            "issuer": &did,
            "issuanceDate": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "type": self.types()?,
            "credentialSubject": self.subject(&did)?
        }))?;

        vc.evidence = self.evidence()?;

        signer.sign_vc(&mut vc).await?;

        Ok(vc)
    }

    // Return the @context contents based enum variant
    fn context(&self) -> Result<String, SchemaError>;

    // Returns the evide
    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError>;

    // Returns the object used in credentialSubject
    fn subject(&self, signer_did: &str) -> Result<String, SchemaError>;

    // Return the types used in credential building.
    fn types(&self) -> Result<Vec<String>, SchemaError>;
}
