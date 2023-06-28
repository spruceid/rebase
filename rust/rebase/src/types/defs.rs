pub use crate::types::{capability::recap::to_action, error::*};
use async_trait::async_trait;
use cacaos::siwe::{generate_nonce, TimeStamp, Version as SIWEVersion};
use chrono::{SecondsFormat, Utc};
use did_ethr::DIDEthr;
use did_ion::DIDION;
use did_jwk::DIDJWK;
pub use did_method_key::DIDKey;
use did_onion::DIDOnion;
use did_pkh::DIDPKH;
use did_tz::DIDTz;
use did_web::DIDWeb;
use did_webkey::DIDWebKey;
use http::uri::Authority;
pub use iri_string::types::UriString;
use libipld::cid::Cid;
use schemars::schema::RootSchema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::{serde_as, DisplayFromStr};
pub use siwe::{eip55, Message};
pub use siwe_recap::Capability;
pub use ssi::{
    did::{DIDMethod, Source, DIDURL},
    did_resolve::{resolve_key, DIDResolver},
    jsonld::ContextLoader,
    jwk::JWK,
    ldp::Proof as LDProof,
    one_or_many::OneOrMany,
    ucan::{Capability as UCanCapability, UcanResource, UcanScope},
    vc::{get_verification_method, Credential, Evidence, LinkedDataProofOptions, URI},
};
pub use ssi_dids::DIDMethods;
use std::collections::BTreeMap;
use ts_rs::TS;
use uuid::Uuid;

use super::enums::attestation::AttestationTypes;

// TODO: Something better? This is C+P from kepler/lib
pub mod address {
    use hex::{encode, FromHex};
    use serde::de::{Deserialize, Deserializer, Error};
    use serde::{Serialize, Serializer};

    pub fn deserialize<'de, D>(d: D) -> Result<[u8; 20], D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(d).and_then(|address| {
            <[u8; 20]>::from_hex(address.strip_prefix("0x").unwrap_or(&address))
                .map_err(|e| D::Error::custom(format!("failed to parse ethereum: {e}")))
        })
    }

    pub fn serialize<S: Serializer>(v: &[u8; 20], s: S) -> Result<S::Ok, S::Error> {
        let h = encode(*v);
        String::serialize(&h, s)
    }
}
#[serde_as]
#[derive(Deserialize, Clone, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct SessionConfig {
    #[serde(with = "crate::types::defs::address")]
    #[ts(type = "string")]
    pub address: [u8; 20],
    pub chain_id: u64,
    #[serde_as(as = "DisplayFromStr")]
    #[ts(type = "string")]
    pub domain: Authority,
    #[serde_as(as = "DisplayFromStr")]
    #[ts(type = "string")]
    pub issued_at: TimeStamp,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    #[ts(type = "string")]
    pub not_before: Option<TimeStamp>,
    #[serde_as(as = "DisplayFromStr")]
    #[ts(type = "string")]
    pub expiration_time: TimeStamp,
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    #[serde(default)]
    #[ts(type = "Array<string>")]
    pub parents: Option<Vec<Cid>>,
    #[serde(default)]
    #[ts(type = "object")]
    pub jwk: Option<JWK>,
}

impl SessionConfig {
    pub async fn generate_message(
        &mut self,
        service_key: &str,
        delegated_capabilities: &Vec<AttestationTypes>,
    ) -> Result<String, RebaseError> {
        if self.jwk.is_none() {
            self.generate_jwk()?;
        }

        let dk = DIDKey {};

        let d =
            // NOTE: This unwrap is safe from the above is_none check
            dk.generate(&Source::Key(&self.jwk.as_ref().unwrap()))
                .ok_or(CapabilityError::ReCapError(
                    "DID Generation returned None".to_string(),
                ))?;

        let vm = get_verification_method(&d, &dk)
            .await
            .ok_or(CapabilityError::ReCapError(
                "Failed to generated verification method from DID".to_string(),
            ))?;

        let m = self.into_message(&vm, service_key, delegated_capabilities)?;
        Ok(m.to_string())
    }

    fn generate_jwk(&mut self) -> Result<(), RebaseError> {
        let key = JWK::generate_ed25519().map_err(|error| {
            CapabilityError::ReCapError(format!("failed to generate session key: {}", error))
        })?;

        self.jwk = Some(key);
        Ok(())
    }

    // This should be accessed through "Generate Message"
    fn into_message(
        &self,
        delegate: &str,
        service_key: &str,
        delegated_capabilities: &Vec<AttestationTypes>,
    ) -> Result<Message, RebaseError> {
        let d: UriString = delegate.try_into().map_err(|_e| {
            CapabilityError::ReCapError(format!(
                "failed to parse delegate into UriString, delegate: {}",
                delegate
            ))
        })?;

        let u: UriString = service_key.try_into().map_err(|_e| {
            CapabilityError::ReCapError(format!(
                "failed to parse witness into UriString, service_key: {}",
                service_key
            ))
        })?;

        // NOTE: If using caveats, _ will need to be some sort of struct.
        let v: Vec<(String, Vec<BTreeMap<String, _>>)> = delegated_capabilities
            .iter()
            .map(|c| (to_action(c), Vec::<BTreeMap<String, _>>::new()))
            .collect();

        let m = Capability::<String>::new()
            .with_actions_convert(u, v)
            .map_err(|e| {
                CapabilityError::ReCapError(format!(
                    "failed to parse generate actions: {}",
                    e.to_string()
                ))
            })?
            .build_message(Message {
                address: self.address,
                chain_id: self.chain_id,
                domain: self.domain.clone(),
                expiration_time: Some(self.expiration_time.clone()),
                issued_at: self.issued_at.clone(),
                nonce: generate_nonce(),
                not_before: self.not_before.clone(),
                request_id: None,
                statement: None,
                resources: vec![],
                uri: d,
                version: SIWEVersion::V1,
            })
            .map_err(|e| {
                CapabilityError::ReCapError(format!(
                    "failed to generate SIWE message: {}",
                    e.to_string()
                ))
            })?;
        Ok(m)
    }
}
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ResolverOpts {
    did_onion_proxy_url: Option<String>,
    did_ion_api_url: Option<String>,
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
