// TODO: Make this less ugly.
use crate::signer::signer::{Signer, SignerMethods, SignerError};
use chrono::{SecondsFormat, Utc};
use serde_json::{json, Error as SeralizeError};
use ssi::{
    one_or_many::OneOrMany,
    vc::{Credential, Evidence}
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum SchemaError {
    #[error("could not create credential context, {0}")]
    Context(String),
    #[error("could not create credential evidence, {0}")]
    Evidence(String),
    #[error("failed to serailize credential, {0}")]
    Serialize(#[from] SeralizeError),
    #[error("failed in signer, {0}")]
    Signer(#[from] SignerError),
    #[error("could not create credential subject, {0}")]
    Subject(String),
    #[error("could not create credential types, {0}")]
    Types(String),
}

pub trait SchemaType {
    // Return the @context contents based enum variant
    fn context(&self) -> Result<String, SchemaError>;

    // Return the types used in credential building.
    fn types(&self) -> Result<Vec<String>, SchemaError>;

    // Returns the object used in credentialSubject
    fn subject(&self, signer_did: &str) -> Result<String, SchemaError>;

    // Returns the evide
    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError>;

    // Return the complete, signed credential
    fn to_credential<T: SignerMethods>(&self, signer: Signer<T>) -> Result<Credential, SchemaError> {
        let did = signer.as_did()?;

        let mut vc: Credential = serde_json::from_value(json!({
            "@context": self.context()?,
            "id": format!("urn:uuid:{}", Uuid::new_v4().to_string()),
            "issuer": &did,
            "issuanceDate": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "type": self.types()?,
            "credentialSubject": self.subject(&did)?
        }))?;

        vc.evidence = self.evidence()?;

        signer.sign_vc(&mut vc)?;

        Ok(vc)
    }
}
