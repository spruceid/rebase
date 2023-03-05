use crate::schema::schema_type::{SchemaError, SchemaType};
use crate::signer::signer::{SignerType, DID as SignerDID};
use crate::witness::{signer_type::SignerTypes, witness::WitnessError};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};

#[derive(Clone, Deserialize, Serialize)]
pub struct Opts {
    pub key_1: SignerDID,
    pub certification: String,
    pub license: String,
    pub certifiedby: String,
    pub date: String,
}

impl Opts {
    pub fn generate_statement(&self) -> Result<String, WitnessError> {
        let key_1 = SignerTypes::new(&self.key_1)?;
        Ok(format!(
            "I am attesting that on the {} I was credentialed by {} where I recevied {} with the license id {}",
            signer_type.date()?,
            signer_type.certifiedby()?,
            signer_type.certification()?,
            signer_type.license()?,
        ))
    }
}

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub statement_opts: Opts,
    pub signature_1: String,
}

impl SchemaType for Claim {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
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
                        "date": "https://example.com/signature_2",
                        "certifiedby": "https://example.com/signature_2",
                        "certification": "https://example.com/signature_2",
                        "license": "https://example.com/signature_2",
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
        let certification = String::new(&self.statement_opts.certification)?;
        let certifiedby = String::new(&self.statement_opts.certifiedby)?;
        let date = String::new(&self.statement_opts.date)?;
        let license = String::new(&self.statement_opts.license)?;
        Ok(json!({
            "id": key_1.did_id()?,
            "date": certification,
            "certifiedby": certifiedby,
            "date": date,
            "license": license,
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
    ) -> Result<Self, WitnessError> {
        let statement = opts.generate_statement()?;
        let key_1 = SignerTypes::new(&opts.key_1)?;
        key_1.valid_signature(&statement, &signature_1).await?;

        Ok(Claim {
            statement_opts: opts,
            signature_1,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signer::ed25519::Ed25519;
    use crate::util::util::{
        test_ed25519_did, test_ed25519_did_2, test_eth_did, test_eth_did_2, test_witness_signature,
        TestKey, TestWitness, TEST_2KEY_ED25519_SIG_1, TEST_2KEY_ED25519_SIG_2,
        TEST_2KEY_ETH_SIG_1, TEST_2KEY_ETH_SIG_2,
    };

    async fn mock_proof(
        key_1: fn() -> SignerDID,
        sig_1: &str,
        sig_2: &str,
    ) -> Result<Claim, WitnessError> {
        Claim::new(
            // TODO: Make test util
            Opts {
                key_1: key_1(),
                key_2: key_2(),
            },
            sig_1.to_owned(),
            sig_2.to_owned(),
        )
        .await
    }

    #[tokio::test]
    async fn test_eth_claim() {
        // The valid case.
        mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_1,
            TEST_2KEY_ETH_SIG_2,
        )
        .await
        .unwrap()
        .unsigned_credential(&Ed25519::new(&test_ed25519_did()).unwrap())
        .await
        .unwrap();

        // Swapped signatures.
        match mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_2,
            TEST_2KEY_ETH_SIG_1,
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed signatures were incorrectly validated!"),
        }

        // Swapped keys.
        match mock_proof(
            test_eth_did_2,
            test_eth_did,
            TEST_2KEY_ETH_SIG_1,
            TEST_2KEY_ETH_SIG_2,
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed keys were incorrectly validated!"),
        }

        // Unrelated signatures one of three.
        match mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_1,
            &test_witness_signature(TestWitness::DNS, TestKey::Eth).unwrap(),
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_2 was incorrectly validated!"),
        }

        // two of three
        match mock_proof(
            test_eth_did,
            test_eth_did_2,
            &test_witness_signature(TestWitness::GitHub, TestKey::Eth).unwrap(),
            TEST_2KEY_ETH_SIG_2,
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_1 was incorrectly validated!"),
        }

        // three of three
        match mock_proof(
            test_eth_did,
            test_eth_did_2,
            TEST_2KEY_ETH_SIG_2,
            &test_witness_signature(TestWitness::Twitter, TestKey::Eth).unwrap(),
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signatures in both signatures were incorrectly validated!"),
        }
    }

    #[tokio::test]
    async fn test_ed25519_claim() {
        // The valid case.
        mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_1,
            TEST_2KEY_ED25519_SIG_2,
        )
        .await
        .unwrap()
        .unsigned_credential(&Ed25519::new(&test_ed25519_did()).unwrap())
        .await
        .unwrap();

        // Swapped signatures.
        match mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_2,
            TEST_2KEY_ED25519_SIG_1,
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed signatures were incorrectly validated!"),
        }

        // Swapped keys.
        match mock_proof(
            test_ed25519_did_2,
            test_ed25519_did,
            TEST_2KEY_ED25519_SIG_1,
            TEST_2KEY_ED25519_SIG_2,
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Reversed keys were incorrectly validated!"),
        }

        // Unrelated signatures one of three.
        match mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_1,
            &test_witness_signature(TestWitness::DNS, TestKey::Ed25519).unwrap(),
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_2 was incorrectly validated!"),
        }

        // two of three
        match mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            &test_witness_signature(TestWitness::GitHub, TestKey::Ed25519).unwrap(),
            TEST_2KEY_ED25519_SIG_2,
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signature in signature_1 was incorrectly validated!"),
        }

        // three of three
        match mock_proof(
            test_ed25519_did,
            test_ed25519_did_2,
            TEST_2KEY_ED25519_SIG_2,
            &test_witness_signature(TestWitness::Twitter, TestKey::Ed25519).unwrap(),
        )
        .await
        {
            Err(_) => {}
            Ok(_) => panic!("Invalid signatures in both signatures were incorrectly validated!"),
        }
    }
}
