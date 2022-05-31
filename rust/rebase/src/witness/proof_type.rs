use crate::witness::{
    dns::Claim as DnsProof, github::Claim as GitHubProof, twitter::Claim as TwitterProof,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ProofTypes {
    #[serde(rename = "dns")]
    Dns(DnsProof),
    #[serde(rename = "github")]
    GitHub(GitHubProof),
    #[serde(rename = "twitter")]
    Twitter(TwitterProof),
}
