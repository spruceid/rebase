pub use rebase::issuer;
use rebase::{
    content::{
        dns::Dns as DnsCtnt, email::Email as EmailCtnt, github::GitHub as GitHubCtnt,
        nft_ownership::NftOwnership as NftOwnershipCtnt,
        poap_ownership::PoapOwnership as PoapOwnershipCtnt, reddit::Reddit as RedditCtnt,
        same::Same as SameCtnt, soundcloud::SoundCloud as SoundCloudCtnt,
        twitter::Twitter as TwitterCtnt,
    },
    flow::{
        dns::DnsFlow, email::SendGridBasic as EmailFlow, github::GitHubFlow,
        nft_ownership::NftOwnership as NftOwnershipFlow,
        poap_ownership::PoapOwnership as PoapOwnershipFlow, reddit::RedditFlow, same::SameFlow,
        soundcloud::SoundCloudFlow, twitter::TwitterFlow,
    },
    proof::{
        email::Email as EmailProof, github::GitHub as GitHubProof,
        nft_ownership::NftOwnership as NftOwnershipProof,
        poap_ownership::PoapOwnership as PoapOwnershipProof, same::Same as SameProof,
        twitter::Twitter as TwitterProof,
    },
    statement::{
        dns::Dns as DnsStmt, email::Email as EmailStmt, github::GitHub as GitHubStmt,
        nft_ownership::NftOwnership as NftOwnershipStmt,
        poap_ownership::PoapOwnership as PoapOwnershipStmt, reddit::Reddit as RedditStmt,
        same::Same as SameStmt, soundcloud::SoundCloud as SoundCloudStmt,
        twitter::Twitter as TwitterStmt,
    },
    types::{
        defs::{
            new_resolver, Content, ContextLoader, Credential, Evidence, Flow, FlowResponse,
            Instructions, Issuer, LinkedDataProofOptions, OneOrMany, Proof, Statement, URI,
        },
        error::{ContentError, FlowError, ProofError, StatementError},
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Deserialize, Serialize)]
pub enum InstructionsType {
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "nft_ownership")]
    NftOwnership,
    #[serde(rename = "poap_ownership")]
    PoapOwnership,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "same")]
    Same,
    #[serde(rename = "soundcloud")]
    SoundCloud,
    #[serde(rename = "twitter")]
    Twitter,
}

#[derive(Deserialize, Serialize)]
pub enum Contents {
    Dns(DnsCtnt),
    Email(EmailCtnt),
    GitHub(GitHubCtnt),
    NftOwnership(NftOwnershipCtnt),
    PoapOwnership(PoapOwnershipCtnt),
    Reddit(RedditCtnt),
    Same(SameCtnt),
    SoundCloud(SoundCloudCtnt),
    Twitter(TwitterCtnt),
}

#[async_trait(?Send)]
impl Content for Contents {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::Dns(x) => x.context(),
            Contents::Email(x) => x.context(),
            Contents::GitHub(x) => x.context(),
            Contents::NftOwnership(x) => x.context(),
            Contents::PoapOwnership(x) => x.context(),
            Contents::Reddit(x) => x.context(),
            Contents::Same(x) => x.context(),
            Contents::SoundCloud(x) => x.context(),
            Contents::Twitter(x) => x.context(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            Contents::Dns(x) => x.evidence(),
            Contents::Email(x) => x.evidence(),
            Contents::GitHub(x) => x.evidence(),
            Contents::NftOwnership(x) => x.evidence(),
            Contents::PoapOwnership(x) => x.evidence(),
            Contents::Reddit(x) => x.evidence(),
            Contents::Same(x) => x.evidence(),
            Contents::SoundCloud(x) => x.evidence(),
            Contents::Twitter(x) => x.evidence(),
        }
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::Dns(x) => x.subject(),
            Contents::Email(x) => x.subject(),
            Contents::GitHub(x) => x.subject(),
            Contents::NftOwnership(x) => x.subject(),
            Contents::PoapOwnership(x) => x.subject(),
            Contents::Reddit(x) => x.subject(),
            Contents::Same(x) => x.subject(),
            Contents::SoundCloud(x) => x.subject(),
            Contents::Twitter(x) => x.subject(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            Contents::Dns(x) => x.types(),
            Contents::Email(x) => x.types(),
            Contents::GitHub(x) => x.types(),
            Contents::NftOwnership(x) => x.types(),
            Contents::PoapOwnership(x) => x.types(),
            Contents::Reddit(x) => x.types(),
            Contents::Same(x) => x.types(),
            Contents::SoundCloud(x) => x.types(),
            Contents::Twitter(x) => x.types(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename = "opts")]
pub enum Statements {
    #[serde(rename = "dns")]
    Dns(DnsStmt),
    #[serde(rename = "email")]
    Email(EmailStmt),
    #[serde(rename = "github")]
    GitHub(GitHubStmt),
    #[serde(rename = "nft_ownership")]
    NftOwnership(NftOwnershipStmt),
    #[serde(rename = "poap_ownership")]
    PoapOwnership(PoapOwnershipStmt),
    #[serde(rename = "reddit")]
    Reddit(RedditStmt),
    #[serde(rename = "same")]
    Same(SameStmt),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudStmt),
    #[serde(rename = "twitter")]
    Twitter(TwitterStmt),
}

impl Statement for Statements {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Statements::Dns(x) => x.generate_statement(),
            Statements::Email(x) => x.generate_statement(),
            Statements::GitHub(x) => x.generate_statement(),
            Statements::NftOwnership(x) => x.generate_statement(),
            Statements::PoapOwnership(x) => x.generate_statement(),
            Statements::Reddit(x) => x.generate_statement(),
            Statements::Same(x) => x.generate_statement(),
            Statements::SoundCloud(x) => x.generate_statement(),
            Statements::Twitter(x) => x.generate_statement(),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "proof")]
pub enum Proofs {
    #[serde(rename = "dns")]
    Dns(DnsStmt),
    #[serde(rename = "email")]
    Email(EmailProof),
    #[serde(rename = "github")]
    GitHub(GitHubProof),
    #[serde(rename = "nft_ownership")]
    NftOwnership(NftOwnershipProof),
    #[serde(rename = "poap_ownership")]
    PoapOwnership(PoapOwnershipProof),
    #[serde(rename = "reddit")]
    Reddit(RedditStmt),
    #[serde(rename = "same")]
    Same(SameProof),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudStmt),
    #[serde(rename = "twitter")]
    Twitter(TwitterProof),
}

impl Statement for Proofs {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Proofs::Dns(x) => x.generate_statement(),
            Proofs::Email(x) => x.generate_statement(),
            Proofs::GitHub(x) => x.generate_statement(),
            Proofs::NftOwnership(x) => x.generate_statement(),
            Proofs::PoapOwnership(x) => x.generate_statement(),
            Proofs::Reddit(x) => x.generate_statement(),
            Proofs::Same(x) => x.generate_statement(),
            Proofs::SoundCloud(x) => x.generate_statement(),
            Proofs::Twitter(x) => x.generate_statement(),
        }
    }
}

impl Proof<Contents> for Proofs {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Contents, ProofError> {
        match self {
            Proofs::Dns(x) => Ok(Contents::Dns(x.to_content(statement, signature)?)),
            Proofs::Email(x) => Ok(Contents::Email(x.to_content(statement, signature)?)),
            Proofs::GitHub(x) => Ok(Contents::GitHub(x.to_content(statement, signature)?)),
            Proofs::NftOwnership(x) => {
                Ok(Contents::NftOwnership(x.to_content(statement, signature)?))
            }
            Proofs::PoapOwnership(x) => {
                Ok(Contents::PoapOwnership(x.to_content(statement, signature)?))
            }
            Proofs::Reddit(x) => Ok(Contents::Reddit(x.to_content(statement, signature)?)),
            Proofs::Same(x) => Ok(Contents::Same(x.to_content(statement, signature)?)),
            Proofs::SoundCloud(x) => Ok(Contents::SoundCloud(x.to_content(statement, signature)?)),
            Proofs::Twitter(x) => Ok(Contents::Twitter(x.to_content(statement, signature)?)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct WitnessFlow {
    dns: Option<DnsFlow>,
    email: Option<EmailFlow>,
    github: Option<GitHubFlow>,
    nft_ownership: Option<NftOwnershipFlow>,
    poap_ownership: Option<PoapOwnershipFlow>,
    reddit: Option<RedditFlow>,
    same: Option<SameFlow>,
    soundcloud: Option<SoundCloudFlow>,
    twitter: Option<TwitterFlow>,
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
            Statements::Dns(s) => match &self.dns {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation("no dns flow configured".to_owned())),
            },
            Statements::Email(s) => match &self.email {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation("no email flow configured".to_owned())),
            },
            Statements::GitHub(s) => match &self.github {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            Statements::NftOwnership(s) => match &self.nft_ownership {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            Statements::PoapOwnership(s) => match &self.poap_ownership {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            Statements::Reddit(s) => match &self.reddit {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            Statements::Same(s) => match &self.same {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            Statements::SoundCloud(s) => match &self.soundcloud {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Statements::Twitter(s) => match &self.twitter {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
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
            Proofs::Dns(p) => match &self.dns {
                Some(x) => Ok(Contents::Dns(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation("no dns flow configured".to_owned())),
            },
            Proofs::Email(p) => match &self.email {
                Some(x) => Ok(Contents::Email(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation("no email flow configured".to_owned())),
            },
            Proofs::GitHub(p) => match &self.github {
                Some(x) => Ok(Contents::GitHub(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            Proofs::NftOwnership(p) => match &self.nft_ownership {
                Some(x) => Ok(Contents::NftOwnership(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            Proofs::PoapOwnership(p) => match &self.poap_ownership {
                Some(x) => Ok(Contents::PoapOwnership(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            Proofs::Reddit(p) => match &self.reddit {
                Some(x) => Ok(Contents::Reddit(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            Proofs::Same(p) => match &self.same {
                Some(x) => Ok(Contents::Same(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            Proofs::SoundCloud(p) => match &self.soundcloud {
                Some(x) => Ok(Contents::SoundCloud(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Proofs::Twitter(p) => match &self.twitter {
                Some(x) => Ok(Contents::Twitter(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct InstructionsReq {
    #[serde(rename = "type")]
    pub instruction_type: InstructionsType,
}

#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    // TODO: Change name?
    pub opts: Statements,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: Proofs,
}

// TODO: Refactor the base names of the structs?
#[derive(Clone, Deserialize, Serialize)]
pub struct WitnessJWTRes {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessLDRes {
    pub credential: Credential,
}

// TODO: Refactor the base names of the structs?
// TODO: Make the request an enum and flatten on serailization.
pub type VerifyJWTReq = WitnessJWTRes;
pub type VerifyLDReq = WitnessLDRes;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct VerifyRes {
    pub success: bool,
}

impl WitnessFlow {
    pub fn get_instructions(&self, t: InstructionsType) -> Result<Instructions, FlowError> {
        match t {
            InstructionsType::Dns => match &self.dns {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no dns flow configured".to_owned())),
            },
            InstructionsType::Email => match &self.email {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no email flow configured".to_owned())),
            },
            InstructionsType::GitHub => match &self.github {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            InstructionsType::NftOwnership => match &self.nft_ownership {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            InstructionsType::PoapOwnership => match &self.poap_ownership {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            InstructionsType::Reddit => match &self.reddit {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            InstructionsType::Same => match &self.same {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            InstructionsType::SoundCloud => match &self.soundcloud {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            InstructionsType::Twitter => match &self.twitter {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
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

    // TODO: Unify these two if the request becomes an enum
    pub async fn handle_verify_credential_req<I: Issuer>(
        &self,
        req: &VerifyLDReq,
        issuer: &I,
    ) -> Result<(), FlowError> {
        let ldpo = LinkedDataProofOptions {
            verification_method: Some(URI::String(issuer.verification_method()?)),
            ..Default::default()
        };

        let res = req
            .credential
            .verify(Some(ldpo), &new_resolver(), &mut ContextLoader::default())
            .await;

        if res.errors.is_empty() {
            Ok(())
        } else {
            let message = res.errors.join(" ");
            Err(FlowError::BadLookup(message))
        }
    }

    // TODO: Unify these two if the request becomes an enum
    pub async fn handle_verify_jwt_req<I: Issuer>(
        &self,
        req: &VerifyJWTReq,
        issuer: &I,
    ) -> Result<(), FlowError> {
        let ldpo = LinkedDataProofOptions {
            verification_method: Some(URI::String(issuer.verification_method()?)),
            ..Default::default()
        };

        let res = Credential::verify_jwt(
            &req.jwt,
            Some(ldpo),
            &new_resolver(),
            &mut ContextLoader::default(),
        )
        .await;

        if res.errors.is_empty() {
            Ok(())
        } else {
            let message = res.errors.join(" ");
            Err(FlowError::BadLookup(message))
        }
    }
}
