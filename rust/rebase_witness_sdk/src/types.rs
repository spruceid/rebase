use rebase::types::defs::get_verification_method;
pub use rebase::{
    content::{
        dns_verification::DnsVerificationContent, email_verification::EmailVerificationContent,
        github_verification::GitHubVerificationContent,
        nft_ownership_verification::NftOwnershipVerificationContent,
        poap_ownership_verification::PoapOwnershipVerificationContent,
        reddit_verification::RedditVerificationContent,
        same_controller_assertion::SameControllerAssertionContent,
        soundcloud_verification::SoundCloudVerificationContent,
        twitter_verification::TwitterVerificationContent,
        witnessed_self_issued::content::WitnessedSelfIssuedContent,
    },
    flow::{
        dns_verification::DnsVerificationFlow,
        email_verification::SendGridBasicFlow as EmailVerificationFlow,
        github_verification::GitHubVerificationFlow,
        nft_ownership_verification::{Alchemy, NftOwnershipVerificationFlow},
        poap_ownership_verification::PoapOwnershipVerificationFlow,
        reddit_verification::RedditVerificationFlow,
        same_controller_assertion::SameControllerAssertionFlow,
        soundcloud_verification::SoundCloudVerificationFlow,
        twitter_verification::TwitterVerificationFlow,
        witnessed_self_issued::WitnessedSelfIssuedFlow,
    },
    issuer,
    proof::{
        email_verification::EmailVerificationProof, github_verification::GitHubVerificationProof,
        nft_ownership_verification::NftOwnershipVerificationProof,
        poap_ownership_verification::PoapOwnershipVerificationProof,
        same_controller_assertion::SameControllerAssertionProof,
        twitter_verification::TwitterVerificationProof,
        witnessed_self_issued::proof::WitnessedSelfIssuedProof,
    },
    statement::{
        dns_verification::DnsVerificationStatement, email_verification::EmailVerificationStatement,
        github_verification::GitHubVerificationStatement,
        nft_ownership_verification::NftOwnershipVerificationStatement,
        poap_ownership_verification::PoapOwnershipVerificationStatement,
        reddit_verification::RedditVerificationStatement,
        same_controller_assertion::SameControllerAssertionStatement,
        soundcloud_verification::SoundCloudVerificationStatement,
        twitter_verification::TwitterVerificationStatement,
        witnessed_self_issued::statement::WitnessedSelfIssuedStatement,
    },
    types::{
        defs::{
            make_resolver, Content, ContextLoader, Credential, DIDMethods, DIDResolver, Evidence,
            Flow, FlowResponse, Instructions, Issuer, LinkedDataProofOptions, OneOrMany, Proof,
            Statement, URI,
        },
        error::{ContentError, FlowError, ProofError, StatementError},
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

// NOTE: If there is a way to write a macro where a enum can derive a trait
// by having each member of the enum impl the trait, this file would become
// just enum defs. I have searched, yet it elludes me. May you find the way.
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
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
    WitnessedSelfIssued,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
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
    WitnessedSelfIssued(WitnessedSelfIssuedContent),
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
            Contents::WitnessedSelfIssued(x) => x.context(),
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
            Contents::WitnessedSelfIssued(x) => x.evidence(),
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
            Contents::WitnessedSelfIssued(x) => x.subject(),
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
            Contents::WitnessedSelfIssued(x) => x.types(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[serde(rename = "opts")]
#[ts(export, rename = "Statements")]
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
    WitnessedSelfIssued(WitnessedSelfIssuedStatement),
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
            Statements::WitnessedSelfIssued(x) => x.generate_statement(),
        }
    }
}

#[derive(Deserialize, Serialize, TS)]
#[serde(rename = "proof")]
#[ts(export, rename = "Proofs")]
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
    WitnessedSelfIssued(WitnessedSelfIssuedProof),
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
            Proofs::WitnessedSelfIssued(x) => x.generate_statement(),
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
            Proofs::WitnessedSelfIssued(x) => Ok(Contents::WitnessedSelfIssued(
                x.to_content(statement, signature)?,
            )),
        }
    }
}

#[derive(Deserialize, Serialize)]
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
    pub witnessed_self_issued: Option<WitnessedSelfIssuedFlow>,
}

#[async_trait(?Send)]
impl Flow<Contents, Statements, Proofs> for WitnessFlow {
    // NOTE: This is unused, currently
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Err(FlowError::Validation("Cannot use generalized Instructions function for generalized witness, use get_instructions".to_owned()))
    }

    async fn statement<I: Issuer>(
        &self,
        stmt: &Statements,
        issuer: &I,
    ) -> Result<FlowResponse, FlowError> {
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
            Statements::WitnessedSelfIssued(s) => match &self.witnessed_self_issued {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
        }
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &Proofs,
        issuer: &I,
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
            Proofs::WitnessedSelfIssued(p) => match &self.witnessed_self_issued {
                Some(x) => Ok(Contents::WitnessedSelfIssued(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
        }
    }
}
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct InstructionsReq {
    #[serde(rename = "type")]
    pub instruction_type: FlowType,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct StatementReq {
    // TODO: Change name?
    pub opts: Statements,
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessReq {
    pub proof: Proofs,
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct JWTWrapper {
    pub jwt: String,
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct CredentialWrapper {
    #[ts(type = "object")]
    pub credential: Credential,
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(untagged)]
pub enum VCWrapper {
    Ld(CredentialWrapper),
    Jwt(JWTWrapper),
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
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
            FlowType::WitnessedSelfIssued => match &self.witnessed_self_issued {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
        }
    }

    pub async fn handle_credential<I: Issuer>(
        &self,
        req: &WitnessReq,
        issuer: &I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(self.credential(&req.proof, issuer).await?))
    }

    pub async fn handle_jwt<I: Issuer>(
        &self,
        req: &WitnessReq,
        issuer: &I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!({ "jwt": self.jwt(&req.proof, issuer).await? }))
    }

    pub async fn handle_instructions(
        &self,
        req: &InstructionsReq,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(self.get_instructions(req.instruction_type.clone())?))
    }

    pub async fn handle_statement<I: Issuer>(
        &self,
        req: &StatementReq,
        issuer: &I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(self.statement(&req.opts, issuer).await?))
    }
}

pub async fn handle_verify(req: &VCWrapper) -> Result<(), FlowError> {
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

    let v_method = get_verification_method(&issuer, &make_resolver()).await;
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
                &make_resolver(),
                &mut ContextLoader::default(),
            )
            .await
        }
        VCWrapper::Ld(r) => {
            r.credential
                .verify(Some(ldpo), &make_resolver(), &mut ContextLoader::default())
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
