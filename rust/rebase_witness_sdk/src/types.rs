pub use rebase::issuer;
use rebase::{
    content::{
        dns::Dns as DnsCtnt, email::Email as EmailCtnt, github::GitHub as GitHubCtnt,
        reddit::Reddit as RedditCtnt, soundcloud::SoundCloud as SoundCloudCtnt,
        twitter::Twitter as TwitterCtnt, two_key::TwoKey as TwoKeyCtnt,
    },
    flow::{
        dns::DnsFlow, email::SendGridBasic as EmailFlow, github::GitHubFlow, reddit::RedditFlow,
        response::PostResponse, soundcloud::SoundCloudFlow, twitter::TwitterFlow,
        two_key::TwoKeyFlow,
    },
    proof::{
        email::Email as EmailProof, github::GitHub as GitHubProof,
        twitter::Twitter as TwitterProof, two_key::TwoKey as TwoKeyProof,
    },
    statement::{
        dns::Dns as DnsStmt, email::Email as EmailStmt, github::GitHub as GitHubStmt,
        reddit::Reddit as RedditStmt, soundcloud::SoundCloud as SoundCloudStmt,
        twitter::Twitter as TwitterStmt, two_key::TwoKey as TwoKeyStmt,
    },
    types::{
        error::{ContentError, FlowError, ProofError, StatementError},
        types::{Content, Flow, Instructions, Issuer, Proof, Statement},
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ssi::{
    one_or_many::OneOrMany,
    vc::{Credential, Evidence},
};

// TODO: Change to use this once we're ready for breaking changes
pub type StatementRes = PostResponse;
/*
#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum StatementRes {
    Basic(BasicResponse),
    Post(PostResponse),
}

impl StatementRes {
    pub fn statement(&self) -> String {
        match self {
            StatementRes::Basic(x) => x.statement.clone(),
            StatementRes::Post(x) => x.statement.clone(),
        }
    }
}
*/

#[derive(Clone, Deserialize, Serialize)]
pub enum InstructionsType {
    #[serde(rename = "dns")]
    Dns,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "soundcloud")]
    SoundCloud,
    #[serde(rename = "twitter")]
    Twitter,
    // TODO: RENAME ONCE ISSUE IS RESOLVED
    #[serde(rename = "self_signed")]
    TwoKey,
}

#[derive(Deserialize, Serialize)]
pub enum Contents {
    Dns(DnsCtnt),
    Email(EmailCtnt),
    GitHub(GitHubCtnt),
    Reddit(RedditCtnt),
    SoundCloud(SoundCloudCtnt),
    Twitter(TwitterCtnt),
    TwoKey(TwoKeyCtnt),
}

#[async_trait(?Send)]
impl Content for Contents {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::Dns(x) => x.context(),
            Contents::Email(x) => x.context(),
            Contents::GitHub(x) => x.context(),
            Contents::Reddit(x) => x.context(),
            Contents::SoundCloud(x) => x.context(),
            Contents::Twitter(x) => x.context(),
            Contents::TwoKey(x) => x.context(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            Contents::Dns(x) => x.evidence(),
            Contents::Email(x) => x.evidence(),
            Contents::GitHub(x) => x.evidence(),
            Contents::Reddit(x) => x.evidence(),
            Contents::SoundCloud(x) => x.evidence(),
            Contents::Twitter(x) => x.evidence(),
            Contents::TwoKey(x) => x.evidence(),
        }
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::Dns(x) => x.subject(),
            Contents::Email(x) => x.subject(),
            Contents::GitHub(x) => x.subject(),
            Contents::Reddit(x) => x.subject(),
            Contents::SoundCloud(x) => x.subject(),
            Contents::Twitter(x) => x.subject(),
            Contents::TwoKey(x) => x.subject(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            Contents::Dns(x) => x.types(),
            Contents::Email(x) => x.types(),
            Contents::GitHub(x) => x.types(),
            Contents::Reddit(x) => x.types(),
            Contents::SoundCloud(x) => x.types(),
            Contents::Twitter(x) => x.types(),
            Contents::TwoKey(x) => x.types(),
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
    #[serde(rename = "reddit")]
    Reddit(RedditStmt),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudStmt),
    #[serde(rename = "twitter")]
    Twitter(TwitterStmt),
    // TODO: CHANGE NAME TO RESULT OF ISSUE
    #[serde(rename = "self_signed")]
    TwoKey(TwoKeyStmt),
}

impl Statement for Statements {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Statements::Dns(x) => x.generate_statement(),
            Statements::Email(x) => x.generate_statement(),
            Statements::GitHub(x) => x.generate_statement(),
            Statements::Reddit(x) => x.generate_statement(),
            Statements::SoundCloud(x) => x.generate_statement(),
            Statements::Twitter(x) => x.generate_statement(),
            Statements::TwoKey(x) => x.generate_statement(),
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
    #[serde(rename = "reddit")]
    Reddit(RedditStmt),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudStmt),
    #[serde(rename = "twitter")]
    Twitter(TwitterProof),
    // TODO: CHANGE NAME TO RESULT OF ISSUE
    #[serde(rename = "self_signed")]
    TwoKey(TwoKeyProof),
}

impl Statement for Proofs {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Proofs::Dns(x) => x.generate_statement(),
            Proofs::Email(x) => x.generate_statement(),
            Proofs::GitHub(x) => x.generate_statement(),
            Proofs::Reddit(x) => x.generate_statement(),
            Proofs::SoundCloud(x) => x.generate_statement(),
            Proofs::Twitter(x) => x.generate_statement(),
            Proofs::TwoKey(x) => x.generate_statement(),
        }
    }
}

impl Proof<Contents> for Proofs {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Contents, ProofError> {
        match self {
            Proofs::Dns(x) => Ok(Contents::Dns(x.to_content(statement, signature)?)),
            Proofs::Email(x) => Ok(Contents::Email(x.to_content(statement, signature)?)),
            Proofs::GitHub(x) => Ok(Contents::GitHub(x.to_content(statement, signature)?)),
            Proofs::Reddit(x) => Ok(Contents::Reddit(x.to_content(statement, signature)?)),
            Proofs::SoundCloud(x) => Ok(Contents::SoundCloud(x.to_content(statement, signature)?)),
            Proofs::Twitter(x) => Ok(Contents::Twitter(x.to_content(statement, signature)?)),
            Proofs::TwoKey(x) => Ok(Contents::TwoKey(x.to_content(statement, signature)?)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct WitnessFlow {
    dns: DnsFlow,
    email: Option<EmailFlow>,
    github: Option<GitHubFlow>,
    reddit: RedditFlow,
    soundcloud: Option<SoundCloudFlow>,
    twitter: Option<TwitterFlow>,
    two_key: TwoKeyFlow,
}

#[async_trait(?Send)]
impl Flow<Contents, Statements, Proofs, StatementRes> for WitnessFlow {
    // NOTE: This is unused, currently
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Err(FlowError::Validation("Cannot use generalized Instructions function for generalized witness, use get_instructions".to_owned()))
    }

    async fn statement<I: Issuer>(
        &self,
        stmt: &Statements,
        issuer: &I,
    ) -> Result<StatementRes, FlowError> {
        match stmt {
            Statements::Dns(s) => Ok(self.dns.statement(&s, issuer).await?),
            Statements::Email(s) => match &self.email {
                Some(x) => Ok(x.statement(&s, issuer).await?),
                None => Err(FlowError::Validation("no email flow configured".to_owned())),
            },
            Statements::GitHub(s) => match &self.github {
                Some(x) => Ok(x.statement(&s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            Statements::Reddit(s) => Ok(self.reddit.statement(&s, issuer).await?),
            Statements::SoundCloud(s) => match &self.soundcloud {
                Some(x) => Ok(x.statement(&s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Statements::Twitter(s) => match &self.twitter {
                Some(x) => Ok(x.statement(&s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            Statements::TwoKey(s) => Ok(self.two_key.statement(&s, issuer).await?),
        }
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &Proofs,
        issuer: &I,
    ) -> Result<Contents, FlowError> {
        match proof {
            Proofs::Dns(p) => Ok(Contents::Dns(self.dns.validate_proof(&p, issuer).await?)),
            Proofs::Email(p) => match &self.email {
                Some(x) => Ok(Contents::Email(x.validate_proof(&p, issuer).await?)),
                None => Err(FlowError::Validation("no email flow configured".to_owned())),
            },
            Proofs::GitHub(p) => match &self.github {
                Some(x) => Ok(Contents::GitHub(x.validate_proof(&p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            Proofs::Reddit(p) => Ok(Contents::Reddit(
                self.reddit.validate_proof(&p, issuer).await?,
            )),
            Proofs::SoundCloud(p) => match &self.soundcloud {
                Some(x) => Ok(Contents::SoundCloud(x.validate_proof(&p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Proofs::Twitter(p) => match &self.twitter {
                Some(x) => Ok(Contents::Twitter(x.validate_proof(&p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            Proofs::TwoKey(p) => Ok(Contents::TwoKey(
                self.two_key.validate_proof(&p, issuer).await?,
            )),
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

#[derive(Clone, Deserialize, Serialize)]
pub struct WitnessJWTRes {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessLDRes {
    pub credential: Credential,
}

impl WitnessFlow {
    pub fn get_instructions(&self, t: InstructionsType) -> Result<Instructions, FlowError> {
        match t {
            InstructionsType::Dns => self.dns.instructions(),
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
            InstructionsType::Reddit => self.reddit.instructions(),
            InstructionsType::SoundCloud => match &self.soundcloud {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            InstructionsType::Twitter => match &self.twitter {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            InstructionsType::TwoKey => self.two_key.instructions(),
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
