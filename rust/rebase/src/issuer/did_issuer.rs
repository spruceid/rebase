use crate::{
    context::context_loader::context_loader,
    types::{
        defs::{
            get_public_jwk_and_algo, get_verification_method, make_resolver, Issuer, ResolverOpts,
            Subject,
        },
        error::{IssuerError, SubjectError},
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use ssi::{
    jwk::JWK,
    ldp::Proof,
    one_or_many::OneOrMany,
    vc::{Credential, LinkedDataProofOptions, URI},
};
use ssi_jws::{sign_bytes, verify_bytes};
use ts_rs::TS;

#[derive(Clone, Deserialize, Serialize, TS)]
#[serde(rename = "did_issuer")]
#[ts(export, rename = "DidIssuer")]
pub struct DidIssuer {
    did: String,
    #[ts(type = "object")]
    jwk: JWK,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolver_opts: Option<ResolverOpts>,
}

impl DidIssuer {
    pub fn new(
        did: String,
        jwk_str: &str,
        resolver_opts: Option<ResolverOpts>,
    ) -> Result<Self, IssuerError> {
        let jwk: JWK = serde_json::from_str(jwk_str)
            .map_err(|e| IssuerError::Internal(format!("deserialization error: {}", e)))?;

        Ok(DidIssuer {
            did,
            jwk,
            resolver_opts,
        })
    }
}

#[async_trait(?Send)]
impl Subject for DidIssuer {
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

#[async_trait(?Send)]
impl Issuer for DidIssuer {
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError> {
        let (_, a) = get_public_jwk_and_algo(&self.did, &self.resolver_opts).await?;
        /*  What I wanted to write:
        let v = sign_bytes(a, plain_text.as_bytes(), &self.jwk)
            .map_err(|e| Err(IssuerError::Sign(e.to_string())))?;
        Ok(String::from_utf8(v).map_err(|e| Err(IssuerError::Sign(e.to_string())))?)
        */

        match sign_bytes(a, plain_text.as_bytes(), &self.jwk) {
            Ok(b) => match String::from_utf8(b) {
                Ok(s) => Ok(s),
                Err(e) => Err(IssuerError::Sign(e.to_string())),
            },
            Err(e) => Err(IssuerError::Sign(e.to_string())),
        }
    }

    async fn sign_vc(&self, credential: &mut Credential) -> Result<(), IssuerError> {
        credential.proof = self.proof(credential).await?;
        Ok(())
    }

    async fn generate_jwt(&self, credential: &Credential) -> Result<String, IssuerError> {
        let r = make_resolver(&self.resolver_opts);
        Ok(credential
            .generate_jwt(
                Some(&self.jwk),
                &LinkedDataProofOptions {
                    checks: None,
                    created: None,
                    eip712_domain: None,
                    type_: None,
                    verification_method: get_verification_method(&self.did, &r)
                        .await
                        .map(URI::String),
                    ..Default::default()
                },
                &r,
            )
            .await
            .map_err(|e| IssuerError::Jwt(format!("Failed to generate JWT: {}", e)))?)
    }

    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<Proof>>, IssuerError> {
        let r = make_resolver(&self.resolver_opts);
        let lpdo = LinkedDataProofOptions {
            verification_method: get_verification_method(&self.did, &r)
                .await
                .map(URI::String),
            ..Default::default()
        };

        Ok(Some(OneOrMany::One(
            credential
                .generate_proof(
                    &self.jwk,
                    &lpdo,
                    &r,
                    &mut context_loader().map_err(|e| IssuerError::Vc(e.to_string()))?,
                )
                .await
                .map_err(|e| IssuerError::Proof(format!("Failed to generate LDP proof: {}", e)))?,
        )))
    }
}
