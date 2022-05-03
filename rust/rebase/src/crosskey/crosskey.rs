use crate::schema::{crosskey::Crosskey, schema_type::SchemaError, schema_type::SchemaType};
use crate::signer::signer::{Signer, SignerType, DID as SignerDID, EIP115, PKH as SignerPKH};
use crate::witness::signer_type::SignerTypes;
use ssi::{one_or_many::OneOrMany, vc::Credential};

pub async fn crosskey_claim<T: SignerType, U: SignerType>(
    statement_generator: &impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
    statement_signer: &dyn Signer<T>,
    vc_signer: &dyn Signer<U>,
) -> Result<(String, String), SchemaError> {
    let statement = statement_generator(statement_signer, vc_signer);
    let signature = statement_signer.sign(&statement).await?;

    Ok((statement, signature))
}

pub async fn crosskey_credential<T: SignerType, U: SignerType>(
    statement_generator: impl Fn(&dyn Signer<T>, &dyn Signer<U>) -> String,
    statement_signer: &dyn Signer<T>,
    vc_signer: &dyn Signer<U>,
) -> Result<Credential, SchemaError> {
    let (statement, signature) =
        crosskey_claim(&statement_generator, statement_signer, vc_signer).await?;

    let schema = Crosskey::new(
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

pub async fn default_crosskey_credential<T: SignerType, U: SignerType>(
    statement_signer: &dyn Signer<T>,
    vc_signer: &dyn Signer<U>,
) -> Result<Credential, SchemaError> {
    crosskey_credential(&default_statement, statement_signer, vc_signer).await
}

pub async fn validate_inner_signature(c: Credential) -> Result<(), SchemaError> {
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
