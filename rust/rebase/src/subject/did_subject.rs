use crate::types::{
    defs::{get_public_jwk_and_algo, ResolverOpts, Subject},
    error::SubjectError,
};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ssi_jws::verify_bytes;
use ts_rs::TS;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[serde(rename = "did_subject")]
#[ts(export, rename = "DidSubject")]
pub struct DidSubject {
    did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolver_opts: Option<ResolverOpts>,
}

impl DidSubject {
    pub fn new(did: String, resolver_opts: Option<ResolverOpts>) -> Self {
        DidSubject { did, resolver_opts }
    }
}

#[async_trait(?Send)]
impl Subject for DidSubject {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(self.did.clone())
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        let (jwk, a) = get_public_jwk_and_algo(&self.did, &self.resolver_opts).await?;

        verify_bytes(a, statement.as_bytes(), &jwk, signature.as_bytes())
            .map_err(|e| SubjectError::Validation(e.to_string()))
    }
}
