use crate::{
    schema::schema_type::SchemaType,
    signer::signer::{Signer, SignerType},
    witness::{
        dns::ClaimGenerator as DnsGen,
        github::ClaimGenerator as GithubGen,
        proof_type::ProofTypes,
        self_signed::{Claim as SelfSignedClaim},
        twitter::ClaimGenerator as TwitterGen,
        witness::{Generator, WitnessError},
    },
};

use ssi::vc::Credential as VC;

pub type Credential = VC;

pub struct WitnessGenerator {
    // DNS and no configuration so is supported by default.
    // SelfSigned only is included for useage in flows.
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

    pub async fn witness_ld<T: SignerType>(
        &self,
        proof: &ProofTypes,
        signer: &dyn Signer<T>,
    ) -> Result<Credential, WitnessError> {
        match proof {
            ProofTypes::Dns(x) => self.dns.credential(x, signer).await,
            ProofTypes::SelfSigned(x) => {
                // Validates inner signature by creating
                let claim = SelfSignedClaim::new(
                    x.statement_opts.clone(),
                    x.signature_1.clone(),
                    x.signature_2.clone(),
                )
                .await?;

                claim
                    .credential(signer)
                    .await
                    .map_err(|e| WitnessError::SchemaError(e))
            }
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

    pub async fn witness_jwt<T: SignerType>(
        &self,
        proof: &ProofTypes,
        signer: &dyn Signer<T>,
    ) -> Result<String, WitnessError> {
        match proof {
            ProofTypes::Dns(x) => self.dns.jwt(x, signer).await,
            ProofTypes::SelfSigned(x) => {
                // Validates inner signature by creating
                let claim = SelfSignedClaim::new(
                    x.statement_opts.clone(),
                    x.signature_1.clone(),
                    x.signature_2.clone(),
                )
                .await?;

                claim
                    .jwt(signer)
                    .await
                    .map_err(|e| WitnessError::SchemaError(e))
            }
            ProofTypes::GitHub(x) => match &self.github {
                Some(gen) => gen.jwt(x, signer).await,
                _ => Err(WitnessError::NoWitnessConfig {
                    claim_type: "github".to_owned(),
                }),
            },
            ProofTypes::Twitter(x) => match &self.twitter {
                Some(gen) => gen.jwt(x, signer).await,
                _ => Err(WitnessError::NoWitnessConfig {
                    claim_type: "twitter".to_owned(),
                }),
            },
        }
    }
}
