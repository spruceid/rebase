use crate::signer::{Signer, SignerMethods};
use chrono::{SecondsFormat, Utc};
use serde_json::json;
use ssi::vc::Credential;
use uuid::Uuid;

pub trait SchemaType {
    // Return the @context contents based enum variant
    // TODO: Use this error
    fn context(&self) -> Result<String, String>;

    // Return the types used in credential building.
    // TODO: Use this error
    fn vc_types(&self) -> Result<Vec<String>, String>;

    // Returns the object used in credentialSubject
    // TODO: Use this error
    fn credential_subject(&self, signer_id: &str) -> Result<String, String>;

    // TODO: Use this error
    // fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, String>;

    // TODO: Use this error
    // fn proof(&self) -> Result<Option<OneOrMany<Proof>>, String>;

    // Return the complete, signed credential
    // TODO: Use this error
    fn to_credential<T: SignerMethods>(&self, signer: Signer<T>) -> Result<Credential, String> {
        let did = signer.as_did()?;
        let mut vc: Credential = serde_json::from_value(json!({
            "@context": self.context()?,
            "id": format!("urn:uuid:{}", Uuid::new_v4().to_string()),
            "issuer": &did,
            "issuanceDate": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "type": self.vc_types()?,
            "credentialSubject": self.credential_subject(&did)?
        }))
        .map_err(|e| e.to_string())?;

        // TODO: Impl
    
        Ok(vc)
    }
}
