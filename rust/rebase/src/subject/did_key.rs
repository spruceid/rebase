use crate::types::{defs::Subject, error::SubjectError};
use async_trait::async_trait;
use did_method_key::DIDKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ssi_dids::{
    did_resolve::{DIDResolver, ResolutionInputMetadata},
    VerificationMethod,
};
use ssi_jws::verify_bytes;
use ts_rs::TS;

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

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        if let (_, Some(d), _) = DIDKey
            .resolve(&self.did, &ResolutionInputMetadata::default())
            .await
        {
            if let Some(vm) = d.verification_method {
                if let Some(VerificationMethod::Map(v_meth)) = vm.first() {
                    if let Some(jwk) = v_meth.public_key_jwk.clone() {
                        if let Some(a) = jwk.get_algorithm() {
                            return verify_bytes(
                                a,
                                statement.as_bytes(),
                                &jwk,
                                signature.as_bytes(),
                            )
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
