pub use rebase::{
    content::{
        attestation::content::AttestationContent,
        delegated_attestation::content::DelegatedAttestationContent,
        dns_verification::DnsVerificationContent, email_verification::EmailVerificationContent,
        github_verification::GitHubVerificationContent,
        nft_ownership_verification::NftOwnershipVerificationContent,
        poap_ownership_verification::PoapOwnershipVerificationContent,
        reddit_verification::RedditVerificationContent,
        same_controller_assertion::SameControllerAssertionContent,
        soundcloud_verification::SoundCloudVerificationContent,
        twitter_verification::TwitterVerificationContent,
    },
    context::context_loader::context_loader,
    flow::{
        attestation::AttestationFlow,
        delegated_attestation::DelegatedAttestationFlow,
        dns_verification::DnsVerificationFlow,
        email_verification::SendGridBasicFlow as EmailVerificationFlow,
        github_verification::GitHubVerificationFlow,
        nft_ownership_verification::{Alchemy, NftOwnershipVerificationFlow},
        poap_ownership_verification::PoapOwnershipVerificationFlow,
        reddit_verification::RedditVerificationFlow,
        same_controller_assertion::SameControllerAssertionFlow,
        soundcloud_verification::SoundCloudVerificationFlow,
        twitter_verification::TwitterVerificationFlow,
    },
    issuer,
    proof::{
        attestation::proof::AttestationProof, delegated_attestation::DelegatedAttestationProof,
        email_verification::EmailVerificationProof, github_verification::GitHubVerificationProof,
        nft_ownership_verification::NftOwnershipVerificationProof,
        poap_ownership_verification::PoapOwnershipVerificationProof,
        same_controller_assertion::SameControllerAssertionProof,
        twitter_verification::TwitterVerificationProof,
    },
    statement::{
        attestation::statement::AttestationStatement, dns_verification::DnsVerificationStatement,
        email_verification::EmailVerificationStatement,
        github_verification::GitHubVerificationStatement,
        nft_ownership_verification::NftOwnershipVerificationStatement,
        poap_ownership_verification::PoapOwnershipVerificationStatement,
        reddit_verification::RedditVerificationStatement,
        same_controller_assertion::SameControllerAssertionStatement,
        soundcloud_verification::SoundCloudVerificationStatement,
        twitter_verification::TwitterVerificationStatement,
    },
    types::{
        defs::{
            get_verification_method, make_resolver, to_action, Capability, Content, ContextLoader,
            Credential, DIDKey, DIDMethod, DIDMethods, DIDResolver, Evidence, Flow, Instructions,
            Issuer, LinkedDataProofOptions, OneOrMany, Proof, ResolverOpts, SessionConfig, Source,
            Statement, StatementResponse, UCanCapability, UcanResource, UcanScope, DIDURL, JWK,
            URI,
        },
        enums::attestation::AttestationTypes,
        error::{CapabilityError, ContentError, FlowError, ProofError, StatementError},
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

// NOTE: If there is a way to write a macro where a enum can derive a trait
// by having each member of the enum impl the trait, this file would become
// just enum defs. I have searched, yet it elludes me. May you find the way.
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum FlowType {
    DnsVerification,
    EmailVerification,
    GitHubVerification,
    NftOwnershipVerification,
    PoapOwnershipVerification,
    RedditVerification,
    SameControllerAssertion,
    SoundCloudVerification,
    TwitterVerification,
    Attestation,
    DelegatedAttestation,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Contents {
    DnsVerification(DnsVerificationContent),
    EmailVerification(EmailVerificationContent),
    GitHubVerification(GitHubVerificationContent),
    NftOwnershipVerification(NftOwnershipVerificationContent),
    PoapOwnershipVerification(PoapOwnershipVerificationContent),
    RedditVerification(RedditVerificationContent),
    SameControllerAssertion(SameControllerAssertionContent),
    SoundCloudVerification(SoundCloudVerificationContent),
    TwitterVerification(TwitterVerificationContent),
    Attestation(AttestationContent),
    DelegatedAttestation(DelegatedAttestationContent),
}

#[async_trait(?Send)]
impl Content for Contents {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.context(),
            Contents::EmailVerification(x) => x.context(),
            Contents::GitHubVerification(x) => x.context(),
            Contents::NftOwnershipVerification(x) => x.context(),
            Contents::PoapOwnershipVerification(x) => x.context(),
            Contents::RedditVerification(x) => x.context(),
            Contents::SameControllerAssertion(x) => x.context(),
            Contents::SoundCloudVerification(x) => x.context(),
            Contents::TwitterVerification(x) => x.context(),
            Contents::Attestation(x) => x.context(),
            Contents::DelegatedAttestation(x) => x.context(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.evidence(),
            Contents::EmailVerification(x) => x.evidence(),
            Contents::GitHubVerification(x) => x.evidence(),
            Contents::NftOwnershipVerification(x) => x.evidence(),
            Contents::PoapOwnershipVerification(x) => x.evidence(),
            Contents::RedditVerification(x) => x.evidence(),
            Contents::SameControllerAssertion(x) => x.evidence(),
            Contents::SoundCloudVerification(x) => x.evidence(),
            Contents::TwitterVerification(x) => x.evidence(),
            Contents::Attestation(x) => x.evidence(),
            Contents::DelegatedAttestation(x) => x.evidence(),
        }
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.subject(),
            Contents::EmailVerification(x) => x.subject(),
            Contents::GitHubVerification(x) => x.subject(),
            Contents::NftOwnershipVerification(x) => x.subject(),
            Contents::PoapOwnershipVerification(x) => x.subject(),
            Contents::RedditVerification(x) => x.subject(),
            Contents::SameControllerAssertion(x) => x.subject(),
            Contents::SoundCloudVerification(x) => x.subject(),
            Contents::TwitterVerification(x) => x.subject(),
            Contents::Attestation(x) => x.subject(),
            Contents::DelegatedAttestation(x) => x.subject(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.types(),
            Contents::EmailVerification(x) => x.types(),
            Contents::GitHubVerification(x) => x.types(),
            Contents::NftOwnershipVerification(x) => x.types(),
            Contents::PoapOwnershipVerification(x) => x.types(),
            Contents::RedditVerification(x) => x.types(),
            Contents::SameControllerAssertion(x) => x.types(),
            Contents::SoundCloudVerification(x) => x.types(),
            Contents::TwitterVerification(x) => x.types(),
            Contents::Attestation(x) => x.types(),
            Contents::DelegatedAttestation(x) => x.types(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Statements {
    DnsVerification(DnsVerificationStatement),
    EmailVerification(EmailVerificationStatement),
    GitHubVerification(GitHubVerificationStatement),
    // NOTE: If adding non-alchemy providers, this will need to change
    // to an enum.
    NftOwnershipVerification(NftOwnershipVerificationStatement),
    PoapOwnershipVerification(PoapOwnershipVerificationStatement),
    RedditVerification(RedditVerificationStatement),
    SameControllerAssertion(SameControllerAssertionStatement),
    SoundCloudVerification(SoundCloudVerificationStatement),
    TwitterVerification(TwitterVerificationStatement),
    Attestation(AttestationStatement),
}

impl Statement for Statements {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Statements::DnsVerification(x) => x.generate_statement(),
            Statements::EmailVerification(x) => x.generate_statement(),
            Statements::GitHubVerification(x) => x.generate_statement(),
            Statements::NftOwnershipVerification(x) => x.generate_statement(),
            Statements::PoapOwnershipVerification(x) => x.generate_statement(),
            Statements::RedditVerification(x) => x.generate_statement(),
            Statements::SameControllerAssertion(x) => x.generate_statement(),
            Statements::SoundCloudVerification(x) => x.generate_statement(),
            Statements::TwitterVerification(x) => x.generate_statement(),
            Statements::Attestation(x) => x.generate_statement(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Proofs {
    DnsVerification(DnsVerificationStatement),
    EmailVerification(EmailVerificationProof),
    GitHubVerification(GitHubVerificationProof),
    // NOTE: If adding non-alchemy providers, this will need to change
    // to an enum.
    NftOwnershipVerification(NftOwnershipVerificationProof),
    PoapOwnershipVerification(PoapOwnershipVerificationProof),
    RedditVerification(RedditVerificationStatement),
    SameControllerAssertion(SameControllerAssertionProof),
    SoundCloudVerification(SoundCloudVerificationStatement),
    TwitterVerification(TwitterVerificationProof),
    Attestation(AttestationProof),
    DelegatedAttestation(DelegatedAttestationProof),
}

impl Statement for Proofs {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Proofs::DnsVerification(x) => x.generate_statement(),
            Proofs::EmailVerification(x) => x.generate_statement(),
            Proofs::GitHubVerification(x) => x.generate_statement(),
            Proofs::NftOwnershipVerification(x) => x.generate_statement(),
            Proofs::PoapOwnershipVerification(x) => x.generate_statement(),
            Proofs::RedditVerification(x) => x.generate_statement(),
            Proofs::SameControllerAssertion(x) => x.generate_statement(),
            Proofs::SoundCloudVerification(x) => x.generate_statement(),
            Proofs::TwitterVerification(x) => x.generate_statement(),
            Proofs::Attestation(x) => x.generate_statement(),
            Proofs::DelegatedAttestation(x) => x.generate_statement(),
        }
    }
}

impl Proof<Contents> for Proofs {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Contents, ProofError> {
        match self {
            Proofs::DnsVerification(x) => Ok(Contents::DnsVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::EmailVerification(x) => Ok(Contents::EmailVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::GitHubVerification(x) => Ok(Contents::GitHubVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::NftOwnershipVerification(x) => Ok(Contents::NftOwnershipVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::PoapOwnershipVerification(x) => Ok(Contents::PoapOwnershipVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::RedditVerification(x) => Ok(Contents::RedditVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::SameControllerAssertion(x) => Ok(Contents::SameControllerAssertion(
                x.to_content(statement, signature)?,
            )),
            Proofs::SoundCloudVerification(x) => Ok(Contents::SoundCloudVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::TwitterVerification(x) => Ok(Contents::TwitterVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::Attestation(x) => {
                Ok(Contents::Attestation(x.to_content(statement, signature)?))
            }
            Proofs::DelegatedAttestation(x) => Ok(Contents::DelegatedAttestation(
                x.to_content(statement, signature)?,
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WitnessFlow {
    pub dns_verification: Option<DnsVerificationFlow>,
    pub email_verification: Option<EmailVerificationFlow>,
    #[serde(rename = "GitHubVerification")]
    pub github_verification: Option<GitHubVerificationFlow>,
    pub nft_ownership_verification: Option<NftOwnershipVerificationFlow>,
    pub poap_ownership_verification: Option<PoapOwnershipVerificationFlow>,
    pub reddit_verification: Option<RedditVerificationFlow>,
    pub same_controller_assertion: Option<SameControllerAssertionFlow>,
    #[serde(rename = "SoundCloudVerification")]
    pub soundcloud_verification: Option<SoundCloudVerificationFlow>,
    pub twitter_verification: Option<TwitterVerificationFlow>,
    pub attestation: Option<AttestationFlow>,
    pub delegated_attestation: Option<DelegatedAttestationFlow>,
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Flow<Contents, Statements, Proofs> for WitnessFlow {
    // NOTE: This is unused, currently
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Err(FlowError::Validation("Cannot use generalized Instructions function for generalized witness, use get_instructions".to_owned()))
    }

    async fn statement<I: Issuer + Send + Clone>(
        &self,
        stmt: Statements,
        issuer: I,
    ) -> Result<StatementResponse, FlowError> {
        match stmt {
            Statements::DnsVerification(s) => match &self.dns_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no dns_verification flow configured".to_owned(),
                )),
            },
            Statements::EmailVerification(s) => match &self.email_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no email_verification flow configured".to_owned(),
                )),
            },
            Statements::GitHubVerification(s) => match &self.github_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no github_verification flow configured".to_owned(),
                )),
            },
            Statements::NftOwnershipVerification(s) => match &self.nft_ownership_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            Statements::PoapOwnershipVerification(s) => match &self.poap_ownership_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            Statements::RedditVerification(s) => match &self.reddit_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            Statements::SameControllerAssertion(s) => match &self.same_controller_assertion {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            Statements::SoundCloudVerification(s) => match &self.soundcloud_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Statements::TwitterVerification(s) => match &self.twitter_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            Statements::Attestation(s) => match &self.attestation {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no attestation flow configured".to_owned(),
                )),
            },
        }
    }

    async fn validate_proof<I: Issuer + Send>(
        &self,
        proof: Proofs,
        issuer: I,
    ) -> Result<Contents, FlowError> {
        match proof {
            Proofs::DnsVerification(p) => match &self.dns_verification {
                Some(x) => Ok(Contents::DnsVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no dns_verification flow configured".to_owned(),
                )),
            },
            Proofs::EmailVerification(p) => match &self.email_verification {
                Some(x) => Ok(Contents::EmailVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no email_verification flow configured".to_owned(),
                )),
            },
            Proofs::GitHubVerification(p) => match &self.github_verification {
                Some(x) => Ok(Contents::GitHubVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            Proofs::NftOwnershipVerification(p) => match &self.nft_ownership_verification {
                Some(x) => Ok(Contents::NftOwnershipVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            Proofs::PoapOwnershipVerification(p) => match &self.poap_ownership_verification {
                Some(x) => Ok(Contents::PoapOwnershipVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            Proofs::RedditVerification(p) => match &self.reddit_verification {
                Some(x) => Ok(Contents::RedditVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            Proofs::SameControllerAssertion(p) => match &self.same_controller_assertion {
                Some(x) => Ok(Contents::SameControllerAssertion(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            Proofs::SoundCloudVerification(p) => match &self.soundcloud_verification {
                Some(x) => Ok(Contents::SoundCloudVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Proofs::TwitterVerification(p) => match &self.twitter_verification {
                Some(x) => Ok(Contents::TwitterVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            Proofs::Attestation(p) => match &self.attestation {
                Some(x) => Ok(Contents::Attestation(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no attestation flow configured".to_owned(),
                )),
            },
            Proofs::DelegatedAttestation(p) => match &self.delegated_attestation {
                Some(x) => Ok(Contents::DelegatedAttestation(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no delegated attesation flow configured".to_owned(),
                )),
            },
        }
    }
}
#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct InstructionsReq {
    #[serde(rename = "type")]
    pub instruction_type: FlowType,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JWTWrapper {
    pub jwt: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct CredentialWrapper {
    pub credential: Credential,
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(untagged)]
pub enum VCWrapper {
    Ld(CredentialWrapper),
    Jwt(JWTWrapper),
}

#[derive(Clone, Debug, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct VerifyRes {
    pub success: bool,
}

impl WitnessFlow {
    pub fn get_instructions(&self, t: FlowType) -> Result<Instructions, FlowError> {
        match t {
            FlowType::DnsVerification => match &self.dns_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no dns flow configured".to_owned())),
            },
            FlowType::EmailVerification => match &self.email_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no email_verification flow configured".to_owned(),
                )),
            },
            FlowType::GitHubVerification => match &self.github_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            FlowType::NftOwnershipVerification => match &self.nft_ownership_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            FlowType::PoapOwnershipVerification => match &self.poap_ownership_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            FlowType::RedditVerification => match &self.reddit_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            FlowType::SameControllerAssertion => match &self.same_controller_assertion {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            FlowType::SoundCloudVerification => match &self.soundcloud_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            FlowType::TwitterVerification => match &self.twitter_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            FlowType::Attestation => match &self.attestation {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no attestation flow configured".to_owned(),
                )),
            },
            FlowType::DelegatedAttestation => match &self.delegated_attestation {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no delegated attestation flow configured".to_owned(),
                )),
            },
        }
    }

    pub async fn handle_ld<I: Issuer + Send + Clone>(
        &self,
        proof: Proofs,
        issuer: I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!({ "credential": self.credential(proof, issuer).await? }))
    }

    pub async fn handle_jwt<I: Issuer + Send + Clone>(
        &self,
        proof: Proofs,
        issuer: I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!({ "jwt": self.jwt(proof, issuer).await? }))
    }

    pub async fn handle_instructions(
        &self,
        req: &InstructionsReq,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(self.get_instructions(req.instruction_type.clone())?))
    }

    pub async fn handle_statement<I: Issuer + Send + Clone>(
        &self,
        statement: Statements,
        issuer: I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(self.statement(statement, issuer).await?))
    }
}

pub async fn handle_verify(
    req: &VCWrapper,
    resolver_opts: &Option<ResolverOpts>,
) -> Result<(), FlowError> {
    let issuer = match &req {
        VCWrapper::Jwt(r) => {
            let c = Credential::from_jwt_unsigned(&r.jwt)
                .map_err(|e| FlowError::Validation(e.to_string()))?;
            if c.issuer.is_none() {
                return Err(FlowError::Validation(
                    "No issuer found in the Credential".to_string(),
                ));
            }

            c.issuer.unwrap().get_id()
        }
        VCWrapper::Ld(r) => {
            if r.credential.issuer.is_none() {
                return Err(FlowError::Validation(
                    "No issuer found in the Credential".to_string(),
                ));
            }

            r.credential.issuer.as_ref().unwrap().get_id()
        }
    };

    let v_method = get_verification_method(&issuer, &make_resolver(resolver_opts)).await;
    if v_method.is_none() {
        return Err(FlowError::Validation(
            "Could not generate verifcation method".to_string(),
        ));
    }
    let vm = v_method.unwrap();

    let ldpo = LinkedDataProofOptions {
        verification_method: Some(URI::String(vm)),
        ..Default::default()
    };

    let res = match req {
        VCWrapper::Jwt(r) => {
            Credential::verify_jwt(
                &r.jwt,
                Some(ldpo),
                &make_resolver(resolver_opts),
                &mut context_loader()?,
            )
            .await
        }
        VCWrapper::Ld(r) => {
            r.credential
                .verify(
                    Some(ldpo),
                    &make_resolver(resolver_opts),
                    &mut context_loader()?,
                )
                .await
        }
    };

    if res.errors.is_empty() {
        Ok(())
    } else {
        let message = res.errors.join(" ");
        Err(FlowError::BadLookup(message))
    }
}
