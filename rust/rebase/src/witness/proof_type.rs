use crate::witness::{
    dns::Claim as DnsProof, github::Claim as GitHubProof, self_signed::Claim as SelfSignedProof, twitter::Claim as TwitterProof,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum ProofTypes {
    #[serde(rename = "dns")]
    Dns(DnsProof),
    #[serde(rename = "github")]
    GitHub(GitHubProof),
    #[serde(rename = "self_signed")]
    SelfSigned(SelfSignedProof),
    #[serde(rename = "twitter")]
    Twitter(TwitterProof),
}
