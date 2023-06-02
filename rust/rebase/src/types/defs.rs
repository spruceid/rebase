use crate::types::error::*;
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use did_ethr::DIDEthr;
use did_jwk::DIDJWK;
use did_method_key::DIDKey;
use did_pkh::DIDPKH;
pub use did_web::DIDWeb;
use did_webkey::DIDWebKey;
use schemars::schema::RootSchema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use ssi::{
    did_resolve::DIDResolver,
    jsonld::ContextLoader,
    ldp::Proof as LDProof,
    one_or_many::OneOrMany,
    vc::{get_verification_method, Credential, Evidence, LinkedDataProofOptions, URI},
};
pub use ssi_dids::DIDMethods;
use ts_rs::TS;
use uuid::Uuid;

pub fn make_resolver() -> DIDMethods<'static> {
    let mut methods = DIDMethods::default();
    methods.insert(Box::new(DIDKey));
    methods.insert(Box::new(DIDEthr));
    methods.insert(Box::new(DIDWeb));
    methods.insert(Box::new(DIDWebKey));
    methods.insert(Box::new(DIDPKH));
    methods.insert(Box::new(DIDJWK));
    // NOTE: Requires the below require additionl configuration,
    // TODO: Enable these!
    // methods.insert(Box::new(DIDTZ.clone()));
    // methods.insert(Box::new(DIDONION.clone()));
    // methods.insert(Box::new(ION.clone()));
    methods
}

#[async_trait(?Send)]
pub trait Subject
where
    Self: Sized,
{
    fn did(&self) -> Result<String, SubjectError>;

    fn display_id(&self) -> Result<String, SubjectError>;

    // TODO: Remove this when we use get_verification_method instead
    fn verification_method(&self) -> Result<String, SubjectError>;

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError>;
}

#[async_trait(?Send)]
pub trait Issuer
where
    Self: Subject,
{
    // sign takes plain text and returns the corresponding signature
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError>;
    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    async fn sign_vc(&self, vc: &mut Credential) -> Result<(), IssuerError>;
    // generate_jwt takes a VC and returns it's formatted as a JWT:
    async fn generate_jwt(&self, vc: &Credential) -> Result<String, IssuerError>;
    // proof returns the linked data proof options for a given issuer type
    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<LDProof>>, IssuerError>;
}

#[async_trait(?Send)]
pub trait Content {
    // Return the unsigned credential using a subject.
    async fn unsigned_credential<T: Subject>(
        &self,
        subject: &T,
    ) -> Result<Credential, ContentError> {
        let did = subject.did()?;

        let mut vc: Credential = serde_json::from_value(json!({
            "@context": self.context()?,
            "id": format!("urn:uuid:{}", Uuid::new_v4()),
            "issuer": &did,
            "issuanceDate": Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true),
            "type": self.types()?,
            "credentialSubject": self.subject()?
        }))?;

        vc.evidence = self.evidence()?;

        Ok(vc)
    }

    // Return the complete, signed LD Proof credential
    async fn credential<T: Issuer>(&self, issuer: &T) -> Result<Credential, ContentError> {
        let mut vc = self.unsigned_credential(issuer).await?;

        issuer.sign_vc(&mut vc).await?;

        Ok(vc)
    }

    // Return a JWT signed credential
    async fn jwt<T: Issuer>(&self, issuer: &T) -> Result<String, ContentError> {
        let vc = self.unsigned_credential(issuer).await?;

        Ok(issuer.generate_jwt(&vc).await?)
    }

    // TODO: Better type?
    // Return the @context contents based enum variant
    fn context(&self) -> Result<serde_json::Value, ContentError>;

    // Returns the evidence entry for the VC
    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError>;

    // TODO: Better type?
    // Returns the object used in credentialSubject
    fn subject(&self) -> Result<serde_json::Value, ContentError>;

    // Return the types used in credential building.
    fn types(&self) -> Result<Vec<String>, ContentError>;
}

pub trait Statement {
    // From the an attestation structure, create an accurate statement for signing.
    fn generate_statement(&self) -> Result<String, StatementError>;
}

pub trait Proof<T>
where
    T: Content,
    Self: Statement,
{
    fn to_content(&self, statement: &str, signature: &str) -> Result<T, ProofError>;
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct Instructions {
    pub statement: String,
    pub signature: String,
    pub witness: String,
    #[ts(type = "object")]
    pub statement_schema: RootSchema,
    #[ts(type = "object")]
    pub witness_schema: RootSchema,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct FlowResponse {
    pub statement: String,
    pub delimiter: Option<String>,
}

#[async_trait(?Send)]
pub trait Flow<C, S, P>
where
    C: Content,
    S: Statement,
    P: Proof<C>,
{
    async fn credential<I: Issuer>(&self, proof: &P, issuer: &I) -> Result<Credential, FlowError> {
        let content = self.validate_proof(proof, issuer).await?;
        Ok(content.credential(issuer).await?)
    }

    fn instructions(&self) -> Result<Instructions, FlowError>;

    async fn jwt<I: Issuer>(&self, proof: &P, issuer: &I) -> Result<String, FlowError> {
        let content = self.validate_proof(proof, issuer).await?;
        Ok(content.jwt(issuer).await?)
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &S,
        issuer: &I,
    ) -> Result<FlowResponse, FlowError>;

    async fn unsigned_credential<Subj: Subject, I: Issuer>(
        &self,
        proof: &P,
        subj: &Subj,
        issuer: &I,
    ) -> Result<Credential, FlowError> {
        let content = self.validate_proof(proof, issuer).await?;
        Ok(content.unsigned_credential(subj).await?)
    }

    async fn validate_proof<I: Issuer>(&self, proof: &P, issuer: &I) -> Result<C, FlowError>;
}

// NOTE: Currently only supports main-nets. Other networks could be added here.
// The serialized string variant is what is used in requests to Alchemy's API.
#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub enum AlchemyNetworks {
    #[serde(rename = "eth-mainnet")]
    EthMainnet,
    #[serde(rename = "polygon-mainnet")]
    PolygonMainnet,
}

impl std::string::ToString for AlchemyNetworks {
    fn to_string(&self) -> String {
        match self {
            AlchemyNetworks::EthMainnet => "eth-mainnet".to_string(),
            AlchemyNetworks::PolygonMainnet => "polygon-mainnet".to_string(),
        }
    }
}
