use crate::types::{defs::Subject, error::SubjectError};
use async_trait::async_trait;
use did_method_key::DIDKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ssi::jwk::Params;
use ssi_dids::{
    did_resolve::{DIDResolver, ResolutionInputMetadata},
    VerificationMethod,
};
use ts_rs::TS;

// TODO: NOT USE THIS, SUPPORT THINGS GENERICALLY!
use ed25519_dalek::{PublicKey, Signature, Verifier};

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "did_key")]
#[ts(export, rename = "DidKey")]
pub struct DidKey {
    did: String,
}

#[async_trait(?Send)]
impl Subject for DidKey {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(self.did.clone())
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    fn verification_method(&self) -> Result<String, SubjectError> {
        let s = self.did.trim_start_matches("did:key:");
        Ok(format!("{}#{}", &self.did, s))
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        if let (_, Some(d), _) = DIDKey
            .resolve(&self.did, &ResolutionInputMetadata::default())
            .await
        {
            if let Some(vm) = d.verification_method {
                if let Some(VerificationMethod::Map(v_meth)) = vm.first() {
                    if let Some(jwk) = v_meth.public_key_jwk.clone() {
                        // TODO: Support all possible keys, this only supports ed25519!
                        if let Params::OKP(o) = jwk.params {
                            let p = PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
                                SubjectError::Validation(format!(
                                    "could not generate public key: {}",
                                    e
                                ))
                            })?;
                            let sig = Signature::from_bytes(
                                &hex::decode(signature)
                                    .map_err(|e| SubjectError::Validation(e.to_string()))?,
                            )
                            .map_err(|e| SubjectError::Validation(e.to_string()))?;

                            let stmt = statement.as_bytes();
                            return p
                                .verify(stmt, &sig)
                                .map_err(|e| SubjectError::Validation(e.to_string()));
                        }
                    }
                }
            }
        };

        Err(SubjectError::Validation(
            "Failed to resolve DIDKey".to_string(),
        ))
    }
}
