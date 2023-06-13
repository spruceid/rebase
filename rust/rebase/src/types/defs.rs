use crate::types::error::*;
use async_trait::async_trait;
use chrono::{SecondsFormat, Utc};
use did_ethr::DIDEthr;
use did_ion::DIDION;
use did_jwk::DIDJWK;
use did_method_key::DIDKey;
use did_onion::DIDOnion;
use did_pkh::DIDPKH;
use did_tz::DIDTz;
use did_web::DIDWeb;
use did_webkey::DIDWebKey;
use schemars::schema::RootSchema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use ssi::{
    did_resolve::{DIDResolver, ResolutionInputMetadata},
    jsonld::ContextLoader,
    jwk::{Algorithm, JWK},
    ldp::Proof as LDProof,
    one_or_many::OneOrMany,
    vc::{get_verification_method, Credential, Evidence, LinkedDataProofOptions, URI},
};
pub use ssi_dids::{DIDMethods, VerificationMethod};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Clone, Deserialize, JsonSchema, Serialize, TS)]
#[ts(export)]
pub struct ResolverOpts {
    did_onion_proxy_url: Option<String>,
    did_ion_api_url: Option<String>,
}

pub async fn get_public_jwk_and_algo(
    did: &str,
    resolver_opts: &Option<ResolverOpts>,
) -> Result<(JWK, Algorithm), SubjectError> {
    let r = make_resolver(resolver_opts);
    if let (_, Some(d), _) = r.resolve(did, &ResolutionInputMetadata::default()).await {
        if let Some(vm) = d.verification_method {
            if let Some(VerificationMethod::Map(v_meth)) = vm.first() {
                if let Some(jwk) = v_meth.public_key_jwk.clone() {
                    if let Some(a) = jwk.get_algorithm() {
                        return Ok((jwk, a));
                    }
                }
            }
        }
    };

    Err(SubjectError::Did(
        "Failed to parse key and algorithm from DID".to_string(),
    ))
}

pub fn make_resolver(opts: &Option<ResolverOpts>) -> DIDMethods<'static> {
    let mut methods = DIDMethods::default();
    methods.insert(Box::new(DIDKey));
    methods.insert(Box::new(DIDEthr));
    methods.insert(Box::new(DIDWeb));
    methods.insert(Box::new(DIDWebKey));
    methods.insert(Box::new(DIDPKH));
    methods.insert(Box::new(DIDJWK));
    methods.insert(Box::<DIDTz>::default());

    if let Some(o) = opts {
        if let Some(u) = o.did_ion_api_url.clone() {
            methods.insert(Box::new(DIDION::new(Some(u))));
        }
        if let Some(u) = o.did_onion_proxy_url.clone() {
            let mut did_onion = DIDOnion::default();
            did_onion.proxy_url = u;
            methods.insert(Box::new(did_onion));
        }
    }
    methods
}

#[async_trait(?Send)]
pub trait Subject
where
    Self: Sized,
{
    fn did(&self) -> Result<String, SubjectError>;

    fn display_id(&self) -> Result<String, SubjectError>;

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
pub struct StatementResponse {
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
    ) -> Result<StatementResponse, FlowError>;

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
