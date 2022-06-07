use crate::witness::{
    dns::Claim as DnsStatement, github::Opts as GitHubStatement,
    self_signed::Opts as SelfSignedStatement, twitter::Opts as TwitterStatement,
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
    #[serde(rename = "self_signed")]
    SelfSigned(SelfSignedStatement),
    #[serde(rename = "twitter")]
    Twitter(TwitterStatement),
}

impl Statement for StatementTypes {
    fn generate_statement(&self) -> Result<String, WitnessError> {
        match &self {
            StatementTypes::Dns(x) => x.generate_statement(),
            StatementTypes::GitHub(x) => x.generate_statement(),
            StatementTypes::SelfSigned(x) => x.generate_statement(),
            StatementTypes::Twitter(x) => x.generate_statement(),
        }
    }

    fn delimitor(&self) -> String {
        match &self {
            StatementTypes::Dns(x) => x.delimitor(),
            StatementTypes::GitHub(x) => x.delimitor(),
            // TODO / NOTE: Should this be an err? Permitted? A value?
            StatementTypes::SelfSigned(x) => String::new(),
            StatementTypes::Twitter(x) => x.delimitor(),
        }
    }

    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        match &self {
            StatementTypes::Dns(x) => x.signer_type(),
            StatementTypes::GitHub(x) => x.signer_type(),
            // TODO: Should this be seperated into a different trait?
            StatementTypes::SelfSigned(_) => Err(SignerError::InvalidId {
                signer_type: "2 key".to_owned(),
                reason: "cannot call signer_type on 2 key statement opts".to_owned(),
            }),
            StatementTypes::Twitter(x) => x.signer_type(),
        }
    }
}
