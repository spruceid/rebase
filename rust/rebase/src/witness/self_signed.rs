use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{SignerType, DID as SignerDID};
use crate::witness::{signer_type::SignerTypes, witness::WitnessError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, Serialize)]
pub struct Opts {
    pub key_1: SignerDID,
    pub key_2: SignerDID,
}

impl Opts {
    pub fn generate_statement(&self) -> Result<String, WitnessError> {
        let key_1 = SignerTypes::new(&self.key_1)?;
        let key_2 = SignerTypes::new(&self.key_2)?;
        Ok(format!(
            "I am attesting that {} {} is linked to {} {}",
            key_1.name(),
            key_1.statement_id()?,
            key_2.name(),
            key_2.statement_id()?
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub statement_opts: Opts,
    pub signature_1: String,
    pub signature_2: String,
}

impl SchemaType for Claim {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // Ok(json!([
        //     "https://www.w3.org/2018/credentials/v1",
        //     {
        //         "SelfSignedControl": "https://example.com/SelfSignedControl",
        //         "id": "https://example.com/id",
        //         "sameAs": "https://example.com/sameAs",
        //         "signature_1": "https://example.com/signature_1",
        //         "signature_2": "https://example.com/signature_2",
        //         "statement": "https://example.com/statement",
        //     }
        // ]))

        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "id": "https://example.com/id",
                "sameAs": "http://schema.org/sameAs",
                "SelfSignedControl": "https://example.com/SelfSignedControl",
                "SelfSignedControlVerification": {
                    "@id": "https://example.com/SelfSignedControlVerification",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "signature_1": "https://example.com/signature_1",
                        "signature_2": "https://example.com/signature_2",
                        "statement": "https://example.com/statement",
                    }
                },
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "signature_1".to_string(),
            serde_json::Value::String(self.signature_1.clone()),
        );

        evidence_map.insert(
            "signature_2".to_string(),
            serde_json::Value::String(self.signature_2.clone()),
        );

        evidence_map.insert(
            "statement".to_string(),
            serde_json::Value::String(self.statement_opts.generate_statement().map_err(|e| {
                SchemaError::BadSubject(format!("could not format statement: {}", e))
            })?),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["SelfSignedControlVerification".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        let key_1 = SignerTypes::new(&self.statement_opts.key_1)?;
        let key_2 = SignerTypes::new(&self.statement_opts.key_2)?;
        Ok(json!({
            "id": key_1.did_id()?,
            "sameAs": key_2.did_id()?,
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(serde_json::from_value(json!([
            "VerifiableCredential",
            "SelfSignedControl",
        ]))?)
    }
}

impl Claim {
    pub async fn new(
        opts: Opts,
        signature_1: String,
        signature_2: String,
    ) -> Result<Self, WitnessError> {
        let statement = opts.generate_statement()?;
        let key_1 = SignerTypes::new(&opts.key_1)?;
        let key_2 = SignerTypes::new(&opts.key_2)?;
        key_1.valid_signature(&statement, &signature_1).await?;
        key_2.valid_signature(&statement, &signature_2).await?;

        Ok(Claim {
            statement_opts: opts,
            signature_1,
            signature_2,
        })
    }

    /* TODO: Re-Impl something like this for both:
    pub async fn verify_inner_signatures(c: Credential) -> Result<(), SchemaError> {
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
                                            "could not parse did pkh eip155".to_string(),
                                        ));
                                    }

                                    SignerTypes::new(&SignerDID::PKH(SignerPKH::EIP155(Some(
                                        EIP155 {
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
        */
}
