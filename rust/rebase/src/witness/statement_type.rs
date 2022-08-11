use crate::{
    signer::signer::SignerError,
    witness::{
        dns::Claim as DnsStatement,
        github::Opts as GitHubStatement,
        reddit::Claim as RedditStatement,
        self_signed::Opts as SelfSignedStatement,
        signer_type::SignerTypes,
        soundcloud::Claim as SoundCloudStatement,
        twitter::Opts as TwitterStatement,
        witness::{Statement, WitnessError},
    },
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
pub enum StatementTypes {
    #[serde(rename = "dns")]
    Dns(DnsStatement),
    #[serde(rename = "github")]
    GitHub(GitHubStatement),
    #[serde(rename = "reddit")]
    Reddit(RedditStatement),
    #[serde(rename = "self_signed")]
    SelfSigned(SelfSignedStatement),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudStatement),
    #[serde(rename = "twitter")]
    Twitter(TwitterStatement),
}

impl Statement for StatementTypes {
    fn generate_statement(&self) -> Result<String, WitnessError> {
        match &self {
            StatementTypes::Dns(x) => x.generate_statement(),
            StatementTypes::GitHub(x) => x.generate_statement(),
            StatementTypes::Reddit(x) => x.generate_statement(),
            StatementTypes::SelfSigned(x) => x.generate_statement(),
            StatementTypes::SoundCloud(x) => x.generate_statement(),
            StatementTypes::Twitter(x) => x.generate_statement(),
        }
    }

    fn delimitor(&self) -> String {
        match &self {
            StatementTypes::Dns(x) => x.delimitor(),
            StatementTypes::GitHub(x) => x.delimitor(),
            StatementTypes::Reddit(x) => x.delimitor(),
            // TODO / NOTE: Should this be an err? Permitted? A value?
            StatementTypes::SelfSigned(_) => String::new(),
            StatementTypes::SoundCloud(x) => x.delimitor(),
            StatementTypes::Twitter(x) => x.delimitor(),
        }
    }

    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        match &self {
            StatementTypes::Dns(x) => x.signer_type(),
            StatementTypes::GitHub(x) => x.signer_type(),
            StatementTypes::Reddit(x) => x.signer_type(),
            // TODO: Should this be seperated into a different trait?
            StatementTypes::SelfSigned(_) => Err(SignerError::InvalidId {
                signer_type: "2 key".to_owned(),
                reason: "cannot call signer_type on 2 key statement opts".to_owned(),
            }),
            StatementTypes::SoundCloud(x) => x.signer_type(),
            StatementTypes::Twitter(x) => x.signer_type(),
        }
    }
}
