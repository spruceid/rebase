use crate::{
    schema::schema_type::{SchemaError, SchemaType},
    signer::signer::{Signer, SignerError, SignerType, DID as SignerDID},
    witness::{
        signer_type::SignerTypes,
        signers::Signers,
        witness::{Generator, Proof, Statement, WitnessError},
    },
};
use async_trait::async_trait;
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{one_or_many::OneOrMany, vc::Evidence};
use url::Url;

/* ABSTRACT TYPES */
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "opts")]
pub struct Opts {
    pub email: String,
    pub key_type: SignerDID,
}

impl Statement for Opts {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "{} is linked to the {} {}",
            self.email,
            signer_type.name(),
            signer_type.statement_id()?
        ))
    }

    fn delimitor(&self) -> String {
        "\n\n".to_string()
    }
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "claim")]
pub struct Claim {
    // NOTE: This is the signature of statement_opts + timestamp signed by the Witness.
    pub auth: String,
    // NOTE: This is the signature of the statement opts signed by the User.
    pub signature: String,
    pub statement_opts: Opts,
    pub timestamp: String,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        self.statement_opts.signer_type()
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        self.statement_opts.generate_statement()
    }

    fn delimitor(&self) -> String {
        self.statement_opts.delimitor()
    }
}

pub struct Schema {
    pub key_type: SignerDID,
    pub statement: String,
    pub signature: String,
    pub email: String,
}

impl Proof for Claim {}

impl SchemaType for Schema {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS POINT ELSEWHERE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "EmailVerification": "https://example.com/EmailVerification",
                "EmailVerificationMessage": {
                    "@id": "https://example.com/EmailVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "email": "https://schema.org/email",
                    }
                }
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "email".to_string(),
            serde_json::Value::String(self.email.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["EmailVerificationMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        let signer_type = SignerTypes::new(&self.key_type)?;
        let signer_did = signer_type
            .did_id()
            .map_err(|e| SchemaError::BadSubject(e.to_string()))?;

        Ok(json!({
            "id": signer_did,
            "sameAs": self.email,
        }))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "EmailVerification".to_owned(),
        ])
    }
}

#[async_trait(?Send)]
pub trait EmailClient {
    // Returns if the proof is valid, like if timestamp is still fresh.
    async fn is_valid(&self, proof: &Claim) -> Result<(), WitnessError>;
    // Generates and sends the email from user supplied opts
    async fn send_email(&self, opts: &Opts) -> Result<(), WitnessError>;
    // Validates the proof against what would've been generated at the email sending step.
    async fn validate_challenge(&self, proof: &Claim) -> Result<(), WitnessError>;
    // Returns generated statement after calling send email
    async fn statement_and_send(&self, opts: &Opts) -> Result<String, WitnessError> {
        let statement = opts.generate_statement()?;
        self.send_email(opts).await?;
        Ok(statement)
    }
}

#[async_trait(?Send)]
impl<T: EmailClient> Generator<Claim, Schema> for T {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        self.is_valid(proof).await?;
        self.validate_challenge(&proof).await?;

        Ok(format!(
            "{}{}{}",
            proof.generate_statement()?,
            proof.delimitor(),
            proof.signature
        ))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        statement: &str,
        signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            email: proof.statement_opts.email.clone(),
            key_type: proof.statement_opts.key_type.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct SendGridBasic {
    api_key: String,
    from_addr: String,
    from_name: String,
    // This is checked for a negative value and errs if one is found
    // Alternative is casting u64 to i64 and risking UB.
    max_elapsed_minutes: i64,
    service_name: String,
    // TODO: Generalize this into serializable enum, link SignerTypes, but signers.
    // NOTE: For now only supports Ed25519DidWebJwk
    signer: Signers,
}

impl SendGridBasic {
    async fn body(&self, opts: &Opts) -> Result<String, WitnessError> {
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let statement = format!("{}:::{}", opts.generate_statement()?, now);
        let challenge = self.signer.sign(&statement).await?;
        Ok(format!("Please paste the following into the challenge input on the witness page used to generate this email:\n\nChallenge:\n\n{}:::{}", challenge, now))
    }

    async fn subject(&self, opts: &Opts) -> Result<String, WitnessError> {
        let st = SignerTypes::new(&opts.key_type)?;
        Ok(format!(
            "Verifying ownership of {} {} for {}",
            st.name(),
            st.did_id()?,
            self.service_name
        ))
    }
}

#[async_trait(?Send)]
impl EmailClient for SendGridBasic {
    async fn is_valid(&self, proof: &Claim) -> Result<(), WitnessError> {
        if self.max_elapsed_minutes <= 0 {
            return Err(WitnessError::BadConfig(
                "Max elapsed minutes must be set to a number greater than 0".to_string(),
            ));
        }

        let now = Utc::now();
        let then = DateTime::parse_from_rfc3339(&proof.timestamp)
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?;
        if now - Duration::minutes(self.max_elapsed_minutes) > then {
            return Err(WitnessError::BadLookup(format!(
                "Validation window has expired"
            )));
        }
        Ok(())
    }

    async fn send_email(&self, opts: &Opts) -> Result<(), WitnessError> {
        let body = self.body(opts).await?;
        let req = json!({
            "personalizations": [{
                    "to": [
                        {
                            "email": opts.email,
                            // TODO: Add name?
                        }
                    ],
                    "subject": self.subject(opts).await?
            }],
            "content": [
                {
                    "type": "text/plain",
                    "value": body,
                }
            ],
            "from": {
                "email": self.from_addr,
                "name": self.from_name,
            }
        });

        let u = Url::parse("https://api.sendgrid.com/v3/mail/send").map_err(|e| {
            WitnessError::BadLookup(format!("Failed to parse email API Url: {}", e.to_string()))
        })?;

        let key_header: HeaderValue =
            format!("Bearer {}", &self.api_key).parse().map_err(|_| {
                WitnessError::BadLookup("Failed to generate authorization header".to_string())
            })?;

        let content_header: HeaderValue = "application/json".parse().map_err(|_| {
            WitnessError::BadLookup("Failed to generate authorization header".to_string())
        })?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, key_header);
        headers.insert(CONTENT_TYPE, content_header);

        let client = reqwest::Client::new();

        client
            .post(u)
            .headers(headers)
            .json(&req)
            .send()
            .await
            .map_err(|e| {
                WitnessError::BadLookup(format!("Could not send email: {}", e.to_string()))
            })?;

        // TODO: Verify the contents of the res?

        Ok(())
    }

    async fn validate_challenge(&self, proof: &Claim) -> Result<(), WitnessError> {
        let statement = format!(
            "{}:::{}",
            proof.statement_opts.generate_statement()?,
            proof.timestamp
        );

        Ok(self.signer.valid_signature(&statement, &proof.auth).await?)
    }
}

/* ENUM */

// This will be a list of supported email clients.
// This could be modified to support different email clients for
// WASM, which doesn't support any traditonal approach, then
// different variants could be hidden behind compilation flags.
// For now, only supporting a custom SendGrid client.
#[derive(Deserialize, Serialize)]
pub enum EmailClients {
    #[serde(rename = "send_grid_basic")]
    SendGridBasic(SendGridBasic),
}

#[async_trait(?Send)]
impl EmailClient for EmailClients {
    async fn is_valid(&self, proof: &Claim) -> Result<(), WitnessError> {
        match self {
            EmailClients::SendGridBasic(client) => client.is_valid(proof).await,
        }
    }
    async fn send_email(&self, opts: &Opts) -> Result<(), WitnessError> {
        match self {
            EmailClients::SendGridBasic(client) => client.send_email(opts).await,
        }
    }

    async fn validate_challenge(&self, proof: &Claim) -> Result<(), WitnessError> {
        match self {
            EmailClients::SendGridBasic(client) => client.validate_challenge(proof).await,
        }
    }
}
