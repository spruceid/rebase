use crate::{
    schema::schema_type::SchemaType,
    signer::signer::{Signer, SignerType},
    witness::{
        dns::ClaimGenerator as DnsGen,
        github::ClaimGenerator as GithubGen,
        proof_type::ProofTypes,
        reddit::ClaimGenerator as RedditGen,
        self_signed::Claim as SelfSignedClaim,
        soundcloud::ClaimGenerator as SoundCloudGen,
        twitter::ClaimGenerator as TwitterGen,
        witness::{Generator, WitnessError},
    },
};

use serde::{Deserialize, Serialize};
use ssi::vc::Credential as VC;

pub type Credential = VC;

pub struct WitnessGenerator {
    // DNS and no configuration so is supported by default.
    // SelfSigned only is included for useage in flows.
    // TODO: Make consistent?
    pub dns: DnsGen,
    pub github: Option<GithubGen>,
    pub reddit: RedditGen,
    pub soundcloud: Option<SoundCloudGen>,
    pub twitter: Option<TwitterGen>,
}

#[derive(Serialize, Deserialize)]
pub struct Opts {
    pub github: Option<GithubGen>,
    pub twitter: Option<TwitterGen>,
    pub soundcloud: Option<SoundCloudGen>,
}

impl WitnessGenerator {
    pub fn new(opts: Opts) -> Self {
        WitnessGenerator {
            dns: DnsGen {},
            github: opts.github,
            reddit: RedditGen {},
            soundcloud: opts.soundcloud,
            twitter: opts.twitter,
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
            ProofTypes::Reddit(x) => self.reddit.credential(x, signer).await,
            ProofTypes::SoundCloud(x) => match &self.soundcloud {
                Some(gen) => gen.credential(x, signer).await,
                _ => Err(WitnessError::NoWitnessConfig {
                    claim_type: "soundcloud".to_owned(),
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
            ProofTypes::Reddit(x) => self.reddit.jwt(x, signer).await,
            ProofTypes::SoundCloud(x) => match &self.soundcloud {
                Some(gen) => gen.jwt(x, signer).await,
                _ => Err(WitnessError::NoWitnessConfig {
                    claim_type: "soundcloud".to_owned(),
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
