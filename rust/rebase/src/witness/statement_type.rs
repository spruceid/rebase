use crate::witness::{
    dns::Claim as DnsStatement, github::Opts as GitHubStatement, twitter::Opts as TwitterStatement,
    witness::Statement,
};

use crate::signer::signer::SignerError;
use crate::witness::signer_type::SignerTypes;
use crate::witness::witness::WitnessError;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum StatementTypes {
    #[serde(rename = "dns")]
    Dns(DnsStatement),
    #[serde(rename = "github")]
    GitHub(GitHubStatement),
    #[serde(rename = "twitter")]
    Twitter(TwitterStatement),
}

impl Statement for StatementTypes {
    fn generate_statement(&self) -> Result<String, WitnessError> {
        match &self {
            StatementTypes::Dns(x) => x.generate_statement(),
            StatementTypes::GitHub(x) => x.generate_statement(),
            StatementTypes::Twitter(x) => x.generate_statement(),
        }
    }

    fn delimitor(&self) -> String {
        match &self {
            StatementTypes::Dns(x) => x.delimitor(),
            StatementTypes::GitHub(x) => x.delimitor(),
            StatementTypes::Twitter(x) => x.delimitor(),
        }
    }

    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        match &self {
            StatementTypes::Dns(x) => x.signer_type(),
            StatementTypes::GitHub(x) => x.signer_type(),
            StatementTypes::Twitter(x) => x.signer_type(),
        }
    }
}
