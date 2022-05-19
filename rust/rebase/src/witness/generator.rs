use crate::{
    signer::signer::{Signer, SignerType},
    witness::{
        dns::ClaimGenerator as DnsGen,
        github::ClaimGenerator as GithubGen,
        proof_type::ProofTypes,
        twitter::ClaimGenerator as TwitterGen,
        witness::{Generator, WitnessError},
    },
};

use ssi::vc::Credential;
pub struct WitnessGenerator {
    // DNS takes no configuration so is supported by default.
    // TODO: Make consistent?
    pub dns: DnsGen,
    pub github: Option<GithubGen>,
    pub twitter: Option<TwitterGen>,
}

impl WitnessGenerator {
    // TODO: Streamline this into opts struct?
    pub fn new(twitter_api_key: Option<String>, user_agent: Option<String>) -> Self {
        WitnessGenerator {
            dns: DnsGen {},
            github: match user_agent {
                Some(s) => Some(GithubGen { user_agent: s }),
                None => None,
            },
            twitter: match twitter_api_key {
                Some(s) => Some(TwitterGen { api_key: s }),
                None => None,
            },
        }
    }

    pub async fn witness<T: SignerType>(
        &self,
        proof: &ProofTypes,
        signer: &dyn Signer<T>,
    ) -> Result<Credential, WitnessError> {
        match proof {
            ProofTypes::Dns(x) => self.dns.credential(x, signer).await,
            ProofTypes::GitHub(x) => match &self.github {
                Some(gen) => gen.credential(x, signer).await,
                _ => Err(WitnessError::NoWitnessConfig {
                    claim_type: "github".to_owned(),
                }),
            },
            ProofTypes::Twitter(x) => match &self.twitter {
                Some(gen) => gen.credential(x, signer).await,
                _ => Err(WitnessError::NoWitnessConfig {
                    claim_type: "twitter".to_owned(),
                }),
            },
        }
    }
}
