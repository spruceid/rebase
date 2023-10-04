use crate::{
    context::context_loader::context_loader,
    types::{
        defs::{DIDKey, Issuer, Subject},
        error::{IssuerError, SubjectError},
    },
};
use async_trait::async_trait;
use did_web::DIDWeb;
use ed25519_dalek::{
    ed25519::signature::Signature, Keypair, PublicKey, SecretKey, Signer as InnerSigner, Verifier,
};

use hex;
use ssi::{
    jwk::{Params, JWK},
    ldp::Proof,
    one_or_many::OneOrMany,
    vc::{Credential, LinkedDataProofOptions, URI},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ed25519Jwk {
    pub did: String,
    pub jwk: JWK,
    pub key_name: String,
}

// NOTE: This Issuer supports "did:web" and "did:key" variants neutrally
impl Ed25519Jwk {
    pub fn new(did: &str, jwk_str: &str, key_name: &str) -> Result<Self, IssuerError> {
        if !did.starts_with("did:web:") && !did.starts_with("did:key:") {
            return Err(IssuerError::Internal(format!(
                "Currently only supports ed25519 keys as did:web or did:key, got: {}",
                did
            )));
        }

        let jwk: JWK = serde_json::from_str(jwk_str)
            .map_err(|e| IssuerError::Internal(format!("deserialization error: {}", e)))?;

        Ok(Ed25519Jwk {
            did: did.to_owned(),
            jwk,
            key_name: key_name.to_owned(),
        })
    }

    pub fn to_keypair(&self) -> Result<Keypair, SubjectError> {
        match &self.jwk.clone().params {
            Params::OKP(o) => match &o.private_key {
                Some(key) => Ok(Keypair {
                    secret: SecretKey::from_bytes(&key.0).map_err(|e| {
                        SubjectError::Validation(format!("failed to generate secret key: {}", e))
                    })?,
                    public: PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
                        SubjectError::Validation(format!("could not generate public key: {}", e))
                    })?,
                }),
                _ => Err(SubjectError::Validation(
                    "could not recover private key from jwk".to_string(),
                )),
            },
            _ => Err(SubjectError::Validation(
                "could not recover private key from jwk".to_string(),
            )),
        }
    }

    pub fn public_key(&self) -> Result<PublicKey, SubjectError> {
        match &self.jwk.params {
            Params::OKP(o) => Ok(PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
                SubjectError::Validation(format!("could not generate public key: {}", e))
            })?),
            _ => Err(SubjectError::Validation(
                "could not recover public key from jwk".to_string(),
            )),
        }
    }
}

#[async_trait(?Send)]
impl Subject for Ed25519Jwk {
    fn did(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    fn display_id(&self) -> Result<String, SubjectError> {
        Ok(self.did.to_owned())
    }

    fn verification_method(&self) -> Result<String, SubjectError> {
        Ok(format!("{}#{}", &self.did, &self.key_name))
    }

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError> {
        let sig = Signature::from_bytes(
            &hex::decode(signature).map_err(|e| SubjectError::Validation(e.to_string()))?,
        )
        .map_err(|e| SubjectError::Validation(e.to_string()))?;

        let stmt = statement.as_bytes();
        let pk = self.public_key()?;

        pk.verify(stmt, &sig)
            .map_err(|e| SubjectError::Validation(e.to_string()))
    }
}

#[async_trait(?Send)]
impl Issuer for Ed25519Jwk {
    // sign takes plain text and returns the corresponding signature
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError> {
        let sig = self.to_keypair()?.sign(plain_text.as_bytes());
        Ok(hex::encode(sig.to_bytes()))
    }

    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    async fn sign_vc(&self, credential: &mut Credential) -> Result<(), IssuerError> {
        credential.proof = self.proof(credential).await?;
        Ok(())
    }

    // generate_jwt takes a VC and returns it's formatted as a JWT:
    async fn generate_jwt(&self, credential: &Credential) -> Result<String, IssuerError> {
        // TODO: Figure out how to generalize over did resolver
        if self.did.starts_with("did:web") {
            Ok(credential
                .generate_jwt(
                    Some(&self.jwk),
                    &LinkedDataProofOptions {
                        checks: None,
                        created: None,
                        eip712_domain: None,
                        type_: None,
                        verification_method: Some(URI::String(format!(
                            "{}#{}",
                            self.did()?,
                            self.key_name
                        ))),
                        ..Default::default()
                    },
                    &DIDWeb,
                )
                .await
                .map_err(|e| IssuerError::Jwt(format!("Failed to generate JWT: {}", e)))?)
        } else if self.did.starts_with("did:key") {
            Ok(credential
                .generate_jwt(
                    Some(&self.jwk),
                    &LinkedDataProofOptions {
                        checks: None,
                        created: None,
                        eip712_domain: None,
                        type_: None,
                        verification_method: Some(URI::String(format!(
                            "{}#{}",
                            self.did()?,
                            self.key_name
                        ))),
                        ..Default::default()
                    },
                    &DIDKey,
                )
                .await
                .map_err(|e| IssuerError::Jwt(format!("Failed to generate JWT: {}", e)))?)
        } else {
            Err(IssuerError::Vc(format!(
                "Expected did:web or did:key, got: {}",
                self.did
            )))
        }
    }

    // proof returns the linked data proof options for a given issuer type
    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<Proof>>, IssuerError> {
        let lpdo = LinkedDataProofOptions {
            verification_method: Some(URI::String(format!("{}#{}", self.did()?, self.key_name))),
            ..Default::default()
        };

        // TODO: Figure out how to generalize over did resolver
        if self.did.starts_with("did:web") {
            Ok(Some(OneOrMany::One(
                credential
                    .generate_proof(
                        &self.jwk,
                        &lpdo,
                        &DIDWeb,
                        &mut context_loader().map_err(|e| IssuerError::Vc(e.to_string()))?,
                    )
                    .await
                    .map_err(|e| {
                        IssuerError::Proof(format!("Failed to generate LDP proof: {}", e))
                    })?,
            )))
        } else if self.did.starts_with("did:key") {
            Ok(Some(OneOrMany::One(
                credential
                    .generate_proof(
                        &self.jwk,
                        &lpdo,
                        &DIDKey,
                        &mut context_loader().map_err(|e| IssuerError::Vc(e.to_string()))?,
                    )
                    .await
                    .map_err(|e| {
                        IssuerError::Proof(format!("Failed to generate LDP proof: {}", e))
                    })?,
            )))
        } else {
            Err(IssuerError::Vc(format!(
                "Expected did:web or did:key, got: {}",
                self.did
            )))
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        proof::attestation::basic_post_attestation::BasicPostAttestationProof,
        statement::attestation::basic_post_attestation::BasicPostAttestationStatement,
        test_util::util::test_did_keypair,
        types::defs::{get_verification_method, make_resolver, Content, Proof, Statement},
    };
    use chrono::{SecondsFormat, Utc};

    #[tokio::test]
    async fn test_ed25519_issuer_and_subject() {
        let (subj1, iss1) = test_did_keypair().await.unwrap();
        let (subj2, iss2) = test_did_keypair().await.unwrap();
        // Test issuer signing and verification.
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let sig1 = iss1.sign(&now).await.unwrap();
        let sig2 = iss2.sign(&now).await.unwrap();

        if sig1 == sig2 {
            panic!("Signatures of keys one and two matched")
        }

        iss1.valid_signature(&now, &sig1).await.unwrap();
        iss2.valid_signature(&now, &sig2).await.unwrap();

        if iss1.valid_signature(&now, &sig2).await.is_ok() {
            panic!("Issuer 1 thought signature 2 was valid")
        }

        if iss2.valid_signature(&now, &sig1).await.is_ok() {
            panic!("Issuer 2 thought signature 1 was valid")
        }

        // Test subject verification
        subj1.valid_signature(&now, &sig1).await.unwrap();
        subj2.valid_signature(&now, &sig2).await.unwrap();

        if subj1.valid_signature(&now, &sig2).await.is_ok() {
            panic!("Subject 1 thought signature 2 was valid")
        }

        if subj2.valid_signature(&now, &sig1).await.is_ok() {
            panic!("Subject 2 thought signature 1 was valid")
        }
    }

    #[tokio::test]
    async fn test_ed25519_credential_issuance() {
        let (subj, iss) = test_did_keypair().await.unwrap();
        let statement = BasicPostAttestationStatement {
            subject: subj.clone(),
            reply_to: None,
            title: "Hello".to_string(),
            body: "World".to_string(),
        };

        let s = statement.generate_statement().unwrap();
        let signature = iss.sign(&s).await.unwrap();

        let proof = BasicPostAttestationProof {
            statement,
            signature: signature.clone(),
        };

        let content = proof.to_content(&s, &signature).unwrap();

        let vc = content.credential(&iss).await.unwrap();

        let vc_iss = vc.issuer.as_ref().unwrap().get_id();

        let vm = get_verification_method(&vc_iss, &make_resolver(&None))
            .await
            .unwrap();

        let ldpo = LinkedDataProofOptions {
            verification_method: Some(URI::String(vm)),
            ..Default::default()
        };

        let res = vc
            .verify(
                Some(ldpo),
                &make_resolver(&None),
                &mut context_loader().unwrap(),
            )
            .await;

        if !res.errors.is_empty() {
            panic!("{}", res.errors.join(", "));
        };

        let jwt = content.jwt(&iss).await.unwrap();
        let c = Credential::from_jwt_unsigned(&jwt)
            .map_err(|e| IssuerError::Vc(e.to_string()))
            .unwrap();

        let vc_iss = c.issuer.as_ref().unwrap().get_id();
        let vm = get_verification_method(&vc_iss, &make_resolver(&None))
            .await
            .unwrap();

        let ldpo = LinkedDataProofOptions {
            verification_method: Some(URI::String(vm)),
            ..Default::default()
        };

        let res = Credential::verify_jwt(
            &jwt,
            Some(ldpo),
            &make_resolver(&None),
            &mut context_loader().unwrap(),
        )
        .await;

        if !res.errors.is_empty() {
            panic!("{}", res.errors.join(", "));
        };
    }
}
