use crate::witness::{
    dns::Claim as DnsProof, 
    github::Claim as GitHubProof, 
    reddit::Claim as RedditProof, 
    self_signed::Claim as SelfSignedProof, 
    soundcloud::Claim as SoundCloudProof,
    twitter::Claim as TwitterProof,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
pub enum ProofTypes {
    #[serde(rename = "dns")]
    Dns(DnsProof),
    #[serde(rename = "github")]
    GitHub(GitHubProof),
    #[serde(rename = "reddit")]
    Reddit(RedditProof),
    #[serde(rename = "self_signed")]
    SelfSigned(SelfSignedProof),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudProof),
    #[serde(rename = "twitter")]
    Twitter(TwitterProof),
}
