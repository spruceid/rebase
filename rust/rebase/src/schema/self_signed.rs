use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{Signer, SignerType, DID as SignerDID, EIP115, PKH as SignerPKH};
use crate::witness::signer_type::SignerTypes;
use serde_json::json;
use ssi::{
    one_or_many::OneOrMany,
    vc::{Credential, Evidence},
};

pub struct SelfSigned {
    pub signature: String,
    // The statement signer as a DID
    pub statement: String,
    pub statement_id: String,
    pub vc_id: String,
}

impl SelfSigned {
    pub async fn new<T: SignerType, U: SignerType>(
        signature: String,
        statement: String,
        statement_generator: &impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
        statement_signer: &dyn Signer<T>,
        vc_signer: &dyn Signer<U>,
    ) -> Result<Self, SchemaError> {
        let s = statement_generator(statement_signer, vc_signer);
        if statement != s {
            return Err(SchemaError::MismatchedStatement(format!(
                "credential statement: '{}' generated from arguments: '{}'",
                statement, s
            )));
        }

        statement_signer
            .valid_signature(&statement, &signature)
            .await?;
        Ok(SelfSigned {
            signature,
            statement,
            statement_id: statement_signer.id(),
            vc_id: vc_signer.id(),
        })
    }
}

impl SchemaType for SelfSigned {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "CrosskeyControl": "https://example.com/CrosskeyControl",
                "controller": "https://example.com/controller",
                "sameAs": "https://example.com/sameAs",
                "statement": "https://example.com/statement",
                "signature": "https://example.com/signature",
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Ok(None)
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "controller": self.vc_id,
            "sameAs": self.statement_id,
            "statement": self.statement,
            "signature": self.signature,
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(serde_json::from_value(json!([
            "VerifiableCredential",
            "CrosskeyControl",
        ]))?)
    }
}

pub async fn self_signed_claim<T: SignerType, U: SignerType>(
    statement_generator: &impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
    statement_signer: &dyn Signer<T>,
    vc_signer: &dyn Signer<U>,
) -> Result<(String, String), SchemaError> {
    let statement = statement_generator(statement_signer, vc_signer);
    let signature = statement_signer.sign(&statement).await?;

    Ok((statement, signature))
}

pub async fn self_signed_credential<T: SignerType, U: SignerType>(
    statement_generator: impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
    statement_signer: &dyn Signer<T>,
    vc_signer: &dyn Signer<U>,
) -> Result<Credential, SchemaError> {
    let (statement, signature) =
        self_signed_claim(&statement_generator, statement_signer, vc_signer).await?;

    let schema = SelfSigned::new(
        signature,
        statement,
        &statement_generator,
        statement_signer,
        vc_signer,
    )
    .await?;

    Ok(schema.credential(vc_signer).await?)
}

pub fn default_statement<T: SignerType, U: SignerType>(
    vc_signer: &dyn Signer<T>,
    statement_signer: &dyn Signer<U>,
) -> String {
    format!(
        "{} {} is linked to {} {}",
        vc_signer.signer_type().name(),
        vc_signer.id(),
        statement_signer.signer_type().name(),
        statement_signer.id()
    )
}

pub async fn default_self_signed_credential<T: SignerType, U: SignerType>(
    statement_signer: &dyn Signer<T>,
    vc_signer: &dyn Signer<U>,
) -> Result<Credential, SchemaError> {
    self_signed_credential(&default_statement, statement_signer, vc_signer).await
}

pub async fn verify_inner_signature(c: Credential) -> Result<(), SchemaError> {
    match c.credential_subject {
        OneOrMany::One(x) => match x.property_set {
            None => Err(SchemaError::BadSubject("expected property set".to_string())),
            Some(ps) => match ps.get("sameAs") {
                None => Err(SchemaError::BadSubject(
                    "could not find sameAs entry".to_string(),
                )),
                Some(same_as) => match ps.get("signature") {
                    Some(sig) => match ps.get("statement") {
                        None => Err(SchemaError::BadSubject(
                            "could not find signature entry".to_string(),
                        )),
                        Some(stmt) => {
                            // TODO: Break into trait or fn
                            // TODO: Figure out why this doesn't work:
                            let s: String = serde_json::from_value(same_as.clone())?;

                            let signer_type = if s.starts_with("did:web") {
                                SignerTypes::new(&SignerDID::Web(Some(s)))?
                            } else if s.starts_with("did:pkh:eip155") {
                                let v: Vec<&str> = s.split(":").collect();
                                if v.len() != 5 {
                                    return Err(SchemaError::BadSubject(
                                        "could not parse did pkh eip115".to_string(),
                                    ));
                                }

                                SignerTypes::new(&SignerDID::PKH(SignerPKH::EIP115(Some(
                                    EIP115 {
                                        address: v[4].to_owned(),
                                        chain_id: v[3].to_owned(),
                                    },
                                ))))?
                            } else {
                                return Err(SchemaError::BadSubject(
                                    "could not find signer type from same_as did".to_string(),
                                ));
                            };

                            let sig: String = serde_json::from_value(sig.clone())?;
                            let stmt: String = serde_json::from_value(stmt.clone())?;
                            // TODO: Replace with dynamic parsing from above comment.
                            signer_type
                                .valid_signature(&stmt, &sig)
                                .await
                                .map_err(|e| SchemaError::Signer(e))
                        }
                    },
                    None => Err(SchemaError::BadSubject(
                        "could not find signature entry".to_string(),
                    )),
                },
            },
        },
        _ => Err(SchemaError::BadSubject(
            "expected One recieved Many".to_string(),
        )),
    }
}
