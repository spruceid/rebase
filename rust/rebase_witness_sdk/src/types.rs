pub use rebase::{
    content::{
        attestation::content::AttestationContent, dns_verification::DnsVerificationContent,
        email_verification::EmailVerificationContent,
        github_verification::GitHubVerificationContent,
        nft_ownership_verification::NftOwnershipVerificationContent,
        poap_ownership_verification::PoapOwnershipVerificationContent,
        reddit_verification::RedditVerificationContent,
        same_controller_assertion::SameControllerAssertionContent,
        soundcloud_verification::SoundCloudVerificationContent,
        twitter_verification::TwitterVerificationContent,
    },
    context::context_loader::context_loader,
    flow::{
        attestation::AttestationFlow,
        dns_verification::DnsVerificationFlow,
        email_verification::SendGridBasicFlow as EmailVerificationFlow,
        github_verification::GitHubVerificationFlow,
        nft_ownership_verification::{Alchemy, NftOwnershipVerificationFlow},
        poap_ownership_verification::PoapOwnershipVerificationFlow,
        reddit_verification::RedditVerificationFlow,
        same_controller_assertion::SameControllerAssertionFlow,
        soundcloud_verification::SoundCloudVerificationFlow,
        twitter_verification::TwitterVerificationFlow,
    },
    issuer,
    proof::{
        attestation::proof::AttestationProof, email_verification::EmailVerificationProof,
        github_verification::GitHubVerificationProof,
        nft_ownership_verification::NftOwnershipVerificationProof,
        poap_ownership_verification::PoapOwnershipVerificationProof,
        same_controller_assertion::SameControllerAssertionProof,
        twitter_verification::TwitterVerificationProof,
    },
    statement::{
        attestation::statement::AttestationStatement, dns_verification::DnsVerificationStatement,
        email_verification::EmailVerificationStatement,
        github_verification::GitHubVerificationStatement,
        nft_ownership_verification::NftOwnershipVerificationStatement,
        poap_ownership_verification::PoapOwnershipVerificationStatement,
        reddit_verification::RedditVerificationStatement,
        same_controller_assertion::SameControllerAssertionStatement,
        soundcloud_verification::SoundCloudVerificationStatement,
        twitter_verification::TwitterVerificationStatement,
    },
    types::{
        defs::{
            get_verification_method, make_resolver, Content, ContextLoader, Credential, DIDMethods,
            DIDResolver, Evidence, Flow, Instructions, Issuer, LinkedDataProofOptions, OneOrMany,
            Proof, ResolverOpts, Statement, StatementResponse, URI,
        },
        error::{ContentError, FlowError, ProofError, StatementError},
    },
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;

// NOTE: If there is a way to write a macro where a enum can derive a trait
// by having each member of the enum impl the trait, this file would become
// just enum defs. I have searched, yet it elludes me. May you find the way.
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub enum FlowType {
    DnsVerification,
    EmailVerification,
    GitHubVerification,
    NftOwnershipVerification,
    PoapOwnershipVerification,
    RedditVerification,
    SameControllerAssertion,
    SoundCloudVerification,
    TwitterVerification,
    Attestation,
    // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
    WitnessedSelfIssued,
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub enum CompatContents {
    WitnessedBasicImage(rebase::content::attestation::basic_image_attestation::BasicImageAttestationContent),
    WitnessedBasicPost(rebase::content::attestation::basic_post_attestation::BasicPostAttestationContent),
    WitnessedBasicProfile(rebase::content::attestation::basic_profile_attestation::BasicProfileAttestationContent),
    WitnessedBasicTag(rebase::content::attestation::basic_tag_attestation::BasicTagAttestationContent),
    WitnessedBookReview(rebase::content::attestation::book_review_attestation::BookReviewAttestationContent),
    WitnessedDappPreferences(rebase::content::attestation::dapp_preferences_attestation::DappPreferencesAttestationContent),
    WitnessedFollow(rebase::content::attestation::follow_attestation::FollowAttestationContent),
    WitnessedLike(rebase::content::attestation::like_attestation::LikeAttestationContent),
    WitnessedProgressBookLink(rebase::content::attestation::progress_book_link_attestation::ProgressBookLinkAttestationContent),
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
impl CompatContents {
    pub fn to_attestation(&self) -> AttestationContent {
        match &self {
            CompatContents::WitnessedBasicImage(x) => {
                AttestationContent::BasicImageAttestation(x.clone())
            }
            CompatContents::WitnessedBasicPost(x) => {
                AttestationContent::BasicPostAttestation(x.clone())
            }
            CompatContents::WitnessedBasicProfile(x) => {
                AttestationContent::BasicProfileAttestation(x.clone())
            }
            CompatContents::WitnessedBasicTag(x) => {
                AttestationContent::BasicTagAttestation(x.clone())
            }
            CompatContents::WitnessedBookReview(x) => {
                AttestationContent::BookReviewAttestation(x.clone())
            }
            CompatContents::WitnessedDappPreferences(x) => {
                AttestationContent::DappPreferencesAttestation(x.clone())
            }
            CompatContents::WitnessedFollow(x) => AttestationContent::FollowAttestation(x.clone()),
            CompatContents::WitnessedLike(x) => AttestationContent::LikeAttestation(x.clone()),
            CompatContents::WitnessedProgressBookLink(x) => {
                AttestationContent::ProgressBookLinkAttestation(x.clone())
            }
        }
    }

    pub fn from_attestation(attestation: AttestationContent) -> CompatContents {
        match attestation {
            AttestationContent::BasicImageAttestation(x) => CompatContents::WitnessedBasicImage(x),
            AttestationContent::BasicPostAttestation(x) => CompatContents::WitnessedBasicPost(x),
            AttestationContent::BasicProfileAttestation(x) => {
                CompatContents::WitnessedBasicProfile(x)
            }
            AttestationContent::BasicTagAttestation(x) => CompatContents::WitnessedBasicTag(x),
            AttestationContent::BookReviewAttestation(x) => CompatContents::WitnessedBookReview(x),
            AttestationContent::DappPreferencesAttestation(x) => {
                CompatContents::WitnessedDappPreferences(x)
            }
            AttestationContent::FollowAttestation(x) => CompatContents::WitnessedFollow(x),
            AttestationContent::LikeAttestation(x) => CompatContents::WitnessedLike(x),
            AttestationContent::ProgressBookLinkAttestation(x) => {
                CompatContents::WitnessedProgressBookLink(x)
            }
        }
    }

    pub fn compat_types(&self) -> Vec<String> {
        match &self {
            CompatContents::WitnessedBasicImage(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedBasicImage".to_owned(),
                ]
            }
            CompatContents::WitnessedBasicPost(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedBasicPost".to_owned(),
                ]
            }
            CompatContents::WitnessedBasicProfile(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedBasicProfile".to_owned(),
                ]
            }
            CompatContents::WitnessedBasicTag(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedBasicTag".to_owned(),
                ]
            }
            CompatContents::WitnessedBookReview(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedBookReview".to_owned(),
                ]
            }
            CompatContents::WitnessedDappPreferences(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedDappPreferences".to_owned(),
                ]
            }
            CompatContents::WitnessedFollow(_) => vec![
                "VerifiableCredential".to_owned(),
                "WitnessedFollow".to_owned(),
            ],
            CompatContents::WitnessedLike(_) => vec![
                "VerifiableCredential".to_owned(),
                "WitnessedLike".to_owned(),
            ],
            CompatContents::WitnessedProgressBookLink(_) => {
                vec![
                    "VerifiableCredential".to_owned(),
                    "WitnessedProgressBookLink".to_owned(),
                ]
            }
        }
    }
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub enum Contents {
    DnsVerification(DnsVerificationContent),
    EmailVerification(EmailVerificationContent),
    GitHubVerification(GitHubVerificationContent),
    NftOwnershipVerification(NftOwnershipVerificationContent),
    PoapOwnershipVerification(PoapOwnershipVerificationContent),
    RedditVerification(RedditVerificationContent),
    SameControllerAssertion(SameControllerAssertionContent),
    SoundCloudVerification(SoundCloudVerificationContent),
    TwitterVerification(TwitterVerificationContent),
    Attestation(AttestationContent),
    // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
    WitnessedSelfIssued(CompatContents),
}

#[async_trait(?Send)]
impl Content for Contents {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.context(),
            Contents::EmailVerification(x) => x.context(),
            Contents::GitHubVerification(x) => x.context(),
            Contents::NftOwnershipVerification(x) => x.context(),
            Contents::PoapOwnershipVerification(x) => x.context(),
            Contents::RedditVerification(x) => x.context(),
            Contents::SameControllerAssertion(x) => x.context(),
            Contents::SoundCloudVerification(x) => x.context(),
            Contents::TwitterVerification(x) => x.context(),
            Contents::Attestation(x) => x.context(),
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Contents::WitnessedSelfIssued(x) => x.to_attestation().context(),
        }
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.evidence(),
            Contents::EmailVerification(x) => x.evidence(),
            Contents::GitHubVerification(x) => x.evidence(),
            Contents::NftOwnershipVerification(x) => x.evidence(),
            Contents::PoapOwnershipVerification(x) => x.evidence(),
            Contents::RedditVerification(x) => x.evidence(),
            Contents::SameControllerAssertion(x) => x.evidence(),
            Contents::SoundCloudVerification(x) => x.evidence(),
            Contents::TwitterVerification(x) => x.evidence(),
            Contents::Attestation(x) => x.evidence(),
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Contents::WitnessedSelfIssued(x) => x.to_attestation().evidence(),
        }
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.subject(),
            Contents::EmailVerification(x) => x.subject(),
            Contents::GitHubVerification(x) => x.subject(),
            Contents::NftOwnershipVerification(x) => x.subject(),
            Contents::PoapOwnershipVerification(x) => x.subject(),
            Contents::RedditVerification(x) => x.subject(),
            Contents::SameControllerAssertion(x) => x.subject(),
            Contents::SoundCloudVerification(x) => x.subject(),
            Contents::TwitterVerification(x) => x.subject(),
            Contents::Attestation(x) => x.subject(),
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Contents::WitnessedSelfIssued(x) => x.to_attestation().subject(),
        }
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        match self {
            Contents::DnsVerification(x) => x.types(),
            Contents::EmailVerification(x) => x.types(),
            Contents::GitHubVerification(x) => x.types(),
            Contents::NftOwnershipVerification(x) => x.types(),
            Contents::PoapOwnershipVerification(x) => x.types(),
            Contents::RedditVerification(x) => x.types(),
            Contents::SameControllerAssertion(x) => x.types(),
            Contents::SoundCloudVerification(x) => x.types(),
            Contents::TwitterVerification(x) => x.types(),
            Contents::Attestation(x) => x.types(),
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Contents::WitnessedSelfIssued(x) => Ok(x.compat_types()),
        }
    }
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub enum CompatStatements {
    WitnessedBasicImage(rebase::statement::attestation::basic_image_attestation::BasicImageAttestationStatement),
    WitnessedBasicPost(rebase::statement::attestation::basic_post_attestation::BasicPostAttestationStatement),
    WitnessedBasicProfile(rebase::statement::attestation::basic_profile_attestation::BasicProfileAttestationStatement),
    WitnessedBasicTag(rebase::statement::attestation::basic_tag_attestation::BasicTagAttestationStatement),
    WitnessedBookReview(rebase::statement::attestation::book_review_attestation::BookReviewAttestationStatement),
    WitnessedDappPreferences(rebase::statement::attestation::dapp_preferences_attestation::DappPreferencesAttestationStatement),
    WitnessedFollow(rebase::statement::attestation::follow_attestation::FollowAttestationStatement),
    WitnessedLike(rebase::statement::attestation::like_attestation::LikeAttestationStatement),
    WitnessedProgressBookLink(rebase::statement::attestation::progress_book_link_attestation::ProgressBookLinkAttestationStatement),
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
impl CompatStatements {
    pub fn to_attestation(&self) -> AttestationStatement {
        match &self {
            CompatStatements::WitnessedBasicImage(x) => {
                AttestationStatement::BasicImageAttestation(x.clone())
            }
            CompatStatements::WitnessedBasicPost(x) => {
                AttestationStatement::BasicPostAttestation(x.clone())
            }
            CompatStatements::WitnessedBasicProfile(x) => {
                AttestationStatement::BasicProfileAttestation(x.clone())
            }
            CompatStatements::WitnessedBasicTag(x) => {
                AttestationStatement::BasicTagAttestation(x.clone())
            }
            CompatStatements::WitnessedBookReview(x) => {
                AttestationStatement::BookReviewAttestation(x.clone())
            }
            CompatStatements::WitnessedDappPreferences(x) => {
                AttestationStatement::DappPreferencesAttestation(x.clone())
            }
            CompatStatements::WitnessedFollow(x) => {
                AttestationStatement::FollowAttestation(x.clone())
            }
            CompatStatements::WitnessedLike(x) => AttestationStatement::LikeAttestation(x.clone()),
            CompatStatements::WitnessedProgressBookLink(x) => {
                AttestationStatement::ProgressBookLinkAttestation(x.clone())
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[serde(rename = "opts")]
#[ts(export, rename = "Statements")]
pub enum Statements {
    DnsVerification(DnsVerificationStatement),
    EmailVerification(EmailVerificationStatement),
    GitHubVerification(GitHubVerificationStatement),
    // NOTE: If adding non-alchemy providers, this will need to change
    // to an enum.
    NftOwnershipVerification(NftOwnershipVerificationStatement),
    PoapOwnershipVerification(PoapOwnershipVerificationStatement),
    RedditVerification(RedditVerificationStatement),
    SameControllerAssertion(SameControllerAssertionStatement),
    SoundCloudVerification(SoundCloudVerificationStatement),
    TwitterVerification(TwitterVerificationStatement),
    Attestation(AttestationStatement),
    // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
    WitnessedSelfIssued(CompatStatements),
}

impl Statement for Statements {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Statements::DnsVerification(x) => x.generate_statement(),
            Statements::EmailVerification(x) => x.generate_statement(),
            Statements::GitHubVerification(x) => x.generate_statement(),
            Statements::NftOwnershipVerification(x) => x.generate_statement(),
            Statements::PoapOwnershipVerification(x) => x.generate_statement(),
            Statements::RedditVerification(x) => x.generate_statement(),
            Statements::SameControllerAssertion(x) => x.generate_statement(),
            Statements::SoundCloudVerification(x) => x.generate_statement(),
            Statements::TwitterVerification(x) => x.generate_statement(),
            Statements::Attestation(x) => x.generate_statement(),
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Statements::WitnessedSelfIssued(x) => x.to_attestation().generate_statement(),
        }
    }
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Deserialize, Serialize, TS, Clone)]
#[ts(export)]
pub enum CompatProofs {
    WitnessedBasicImage(rebase::proof::attestation::basic_image_attestation::BasicImageAttestationProof),
    WitnessedBasicPost(rebase::proof::attestation::basic_post_attestation::BasicPostAttestationProof),
    WitnessedBasicProfile(rebase::proof::attestation::basic_profile_attestation::BasicProfileAttestationProof),
    WitnessedBasicTag(rebase::proof::attestation::basic_tag_attestation::BasicTagAttestationProof),
    WitnessedBookReview(rebase::proof::attestation::book_review_attestation::BookReviewAttestationProof),
    WitnessedDappPreferences(rebase::proof::attestation::dapp_preferences_attestation::DappPreferencesAttestationProof),
    WitnessedFollow(rebase::proof::attestation::follow_attestation::FollowAttestationProof),
    WitnessedLike(rebase::proof::attestation::like_attestation::LikeAttestationProof),
    WitnessedProgressBookLink(rebase::proof::attestation::progress_book_link_attestation::ProgressBookLinkAttestationProof),
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
impl CompatProofs {
    pub fn to_attestation(&self) -> AttestationProof {
        match &self {
            CompatProofs::WitnessedBasicImage(x) => {
                AttestationProof::BasicImageAttestation(x.clone())
            }
            CompatProofs::WitnessedBasicPost(x) => {
                AttestationProof::BasicPostAttestation(x.clone())
            }
            CompatProofs::WitnessedBasicProfile(x) => {
                AttestationProof::BasicProfileAttestation(x.clone())
            }
            CompatProofs::WitnessedBasicTag(x) => AttestationProof::BasicTagAttestation(x.clone()),
            CompatProofs::WitnessedBookReview(x) => {
                AttestationProof::BookReviewAttestation(x.clone())
            }
            CompatProofs::WitnessedDappPreferences(x) => {
                AttestationProof::DappPreferencesAttestation(x.clone())
            }
            CompatProofs::WitnessedFollow(x) => AttestationProof::FollowAttestation(x.clone()),
            CompatProofs::WitnessedLike(x) => AttestationProof::LikeAttestation(x.clone()),
            CompatProofs::WitnessedProgressBookLink(x) => {
                AttestationProof::ProgressBookLinkAttestation(x.clone())
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[serde(rename = "proof")]
#[ts(export, rename = "Proofs")]
pub enum Proofs {
    DnsVerification(DnsVerificationStatement),
    EmailVerification(EmailVerificationProof),
    GitHubVerification(GitHubVerificationProof),
    // NOTE: If adding non-alchemy providers, this will need to change
    // to an enum.
    NftOwnershipVerification(NftOwnershipVerificationProof),
    PoapOwnershipVerification(PoapOwnershipVerificationProof),
    RedditVerification(RedditVerificationStatement),
    SameControllerAssertion(SameControllerAssertionProof),
    SoundCloudVerification(SoundCloudVerificationStatement),
    TwitterVerification(TwitterVerificationProof),
    Attestation(AttestationProof),
    // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
    WitnessedSelfIssued(CompatProofs),
}

impl Statement for Proofs {
    fn generate_statement(&self) -> Result<String, StatementError> {
        match &self {
            Proofs::DnsVerification(x) => x.generate_statement(),
            Proofs::EmailVerification(x) => x.generate_statement(),
            Proofs::GitHubVerification(x) => x.generate_statement(),
            Proofs::NftOwnershipVerification(x) => x.generate_statement(),
            Proofs::PoapOwnershipVerification(x) => x.generate_statement(),
            Proofs::RedditVerification(x) => x.generate_statement(),
            Proofs::SameControllerAssertion(x) => x.generate_statement(),
            Proofs::SoundCloudVerification(x) => x.generate_statement(),
            Proofs::TwitterVerification(x) => x.generate_statement(),
            Proofs::Attestation(x) => x.generate_statement(),
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Proofs::WitnessedSelfIssued(x) => x.to_attestation().generate_statement(),
        }
    }
}

impl Proof<Contents> for Proofs {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Contents, ProofError> {
        match self {
            Proofs::DnsVerification(x) => Ok(Contents::DnsVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::EmailVerification(x) => Ok(Contents::EmailVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::GitHubVerification(x) => Ok(Contents::GitHubVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::NftOwnershipVerification(x) => Ok(Contents::NftOwnershipVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::PoapOwnershipVerification(x) => Ok(Contents::PoapOwnershipVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::RedditVerification(x) => Ok(Contents::RedditVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::SameControllerAssertion(x) => Ok(Contents::SameControllerAssertion(
                x.to_content(statement, signature)?,
            )),
            Proofs::SoundCloudVerification(x) => Ok(Contents::SoundCloudVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::TwitterVerification(x) => Ok(Contents::TwitterVerification(
                x.to_content(statement, signature)?,
            )),
            Proofs::Attestation(x) => {
                Ok(Contents::Attestation(x.to_content(statement, signature)?))
            }
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Proofs::WitnessedSelfIssued(x) => Ok(Contents::Attestation(
                x.to_attestation().to_content(statement, signature)?,
            )),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct WitnessFlow {
    pub dns_verification: Option<DnsVerificationFlow>,
    pub email_verification: Option<EmailVerificationFlow>,
    #[serde(rename = "GitHubVerification")]
    pub github_verification: Option<GitHubVerificationFlow>,
    pub nft_ownership_verification: Option<NftOwnershipVerificationFlow>,
    pub poap_ownership_verification: Option<PoapOwnershipVerificationFlow>,
    pub reddit_verification: Option<RedditVerificationFlow>,
    pub same_controller_assertion: Option<SameControllerAssertionFlow>,
    #[serde(rename = "SoundCloudVerification")]
    pub soundcloud_verification: Option<SoundCloudVerificationFlow>,
    pub twitter_verification: Option<TwitterVerificationFlow>,
    pub attestation: Option<AttestationFlow>,
}

#[async_trait(?Send)]
impl Flow<Contents, Statements, Proofs> for WitnessFlow {
    // NOTE: This is unused, currently
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Err(FlowError::Validation("Cannot use generalized Instructions function for generalized witness, use get_instructions".to_owned()))
    }

    async fn statement<I: Issuer>(
        &self,
        stmt: &Statements,
        issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        match stmt {
            Statements::DnsVerification(s) => match &self.dns_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no dns_verification flow configured".to_owned(),
                )),
            },
            Statements::EmailVerification(s) => match &self.email_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no email_verification flow configured".to_owned(),
                )),
            },
            Statements::GitHubVerification(s) => match &self.github_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no github_verification flow configured".to_owned(),
                )),
            },
            Statements::NftOwnershipVerification(s) => match &self.nft_ownership_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            Statements::PoapOwnershipVerification(s) => match &self.poap_ownership_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            Statements::RedditVerification(s) => match &self.reddit_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            Statements::SameControllerAssertion(s) => match &self.same_controller_assertion {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            Statements::SoundCloudVerification(s) => match &self.soundcloud_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Statements::TwitterVerification(s) => match &self.twitter_verification {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            Statements::Attestation(s) => match &self.attestation {
                Some(x) => Ok(x.statement(s, issuer).await?),
                None => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Statements::WitnessedSelfIssued(s) => match &self.attestation {
                Some(x) => Ok(x.statement(&s.to_attestation(), issuer).await?),
                None => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
        }
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &Proofs,
        issuer: &I,
    ) -> Result<Contents, FlowError> {
        match proof {
            Proofs::DnsVerification(p) => match &self.dns_verification {
                Some(x) => Ok(Contents::DnsVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no dns_verification flow configured".to_owned(),
                )),
            },
            Proofs::EmailVerification(p) => match &self.email_verification {
                Some(x) => Ok(Contents::EmailVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no email_verification flow configured".to_owned(),
                )),
            },
            Proofs::GitHubVerification(p) => match &self.github_verification {
                Some(x) => Ok(Contents::GitHubVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            Proofs::NftOwnershipVerification(p) => match &self.nft_ownership_verification {
                Some(x) => Ok(Contents::NftOwnershipVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            Proofs::PoapOwnershipVerification(p) => match &self.poap_ownership_verification {
                Some(x) => Ok(Contents::PoapOwnershipVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            Proofs::RedditVerification(p) => match &self.reddit_verification {
                Some(x) => Ok(Contents::RedditVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            Proofs::SameControllerAssertion(p) => match &self.same_controller_assertion {
                Some(x) => Ok(Contents::SameControllerAssertion(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            Proofs::SoundCloudVerification(p) => match &self.soundcloud_verification {
                Some(x) => Ok(Contents::SoundCloudVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            Proofs::TwitterVerification(p) => match &self.twitter_verification {
                Some(x) => Ok(Contents::TwitterVerification(
                    x.validate_proof(p, issuer).await?,
                )),
                None => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            Proofs::Attestation(p) => match &self.attestation {
                Some(x) => Ok(Contents::Attestation(x.validate_proof(p, issuer).await?)),
                None => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
            // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
            Proofs::WitnessedSelfIssued(p) => match &self.attestation {
                Some(x) => Ok(Contents::WitnessedSelfIssued(
                    CompatContents::from_attestation(
                        x.validate_proof(&p.to_attestation(), issuer).await?,
                    ),
                )),
                None => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
        }
    }
}
#[derive(Deserialize, Serialize, TS)]
#[ts(export)]
pub struct InstructionsReq {
    #[serde(rename = "type")]
    pub instruction_type: FlowType,
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct StatementReq {
    pub opts: Statements,
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(untagged)]
pub enum CompatStatementReq {
    R(StatementReq),
    S(Statements),
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
impl CompatStatementReq {
    pub fn to_statement(&self) -> Statements {
        match self {
            CompatStatementReq::R(r) => r.opts.clone(),
            CompatStatementReq::S(s) => s.clone(),
        }
    }
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct WitnessReq {
    pub proof: Proofs,
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(untagged)]
pub enum CompatWitnessReq {
    R(WitnessReq),
    P(Proofs),
}

// TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
impl CompatWitnessReq {
    pub fn to_proofs(&self) -> Proofs {
        match self {
            CompatWitnessReq::R(r) => r.proof.clone(),
            CompatWitnessReq::P(p) => p.clone(),
        }
    }
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct JWTWrapper {
    pub jwt: String,
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct CredentialWrapper {
    #[ts(type = "object")]
    pub credential: Credential,
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(untagged)]
pub enum VCWrapper {
    Ld(CredentialWrapper),
    Jwt(JWTWrapper),
}

#[derive(Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct VerifyRes {
    pub success: bool,
}

impl WitnessFlow {
    pub fn get_instructions(&self, t: FlowType) -> Result<Instructions, FlowError> {
        match t {
            FlowType::DnsVerification => match &self.dns_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no dns flow configured".to_owned())),
            },
            FlowType::EmailVerification => match &self.email_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no email_verification flow configured".to_owned(),
                )),
            },
            FlowType::GitHubVerification => match &self.github_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no github flow configured".to_owned(),
                )),
            },
            FlowType::NftOwnershipVerification => match &self.nft_ownership_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no nft_ownership flow configured".to_owned(),
                )),
            },
            FlowType::PoapOwnershipVerification => match &self.poap_ownership_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no poap_ownership flow configured".to_owned(),
                )),
            },
            FlowType::RedditVerification => match &self.reddit_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no reddit flow configured".to_owned(),
                )),
            },
            FlowType::SameControllerAssertion => match &self.same_controller_assertion {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation("no same flow configured".to_owned())),
            },
            FlowType::SoundCloudVerification => match &self.soundcloud_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no soundcloud flow configured".to_owned(),
                )),
            },
            FlowType::TwitterVerification => match &self.twitter_verification {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no twitter flow configured".to_owned(),
                )),
            },
            FlowType::Attestation => match &self.attestation {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
            FlowType::WitnessedSelfIssued => match &self.attestation {
                Some(x) => x.instructions(),
                _ => Err(FlowError::Validation(
                    "no witnessed self issued flow configured".to_owned(),
                )),
            },
        }
    }

    pub async fn handle_ld<I: Issuer>(
        &self,
        // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
        proof: &CompatWitnessReq,
        issuer: &I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!({ "credential": self.credential(&proof.to_proofs(), issuer).await? }))
    }

    pub async fn handle_jwt<I: Issuer>(
        &self,
        // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
        proof: &CompatWitnessReq,
        issuer: &I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!({ "jwt": self.jwt(&proof.to_proofs(), issuer).await? }))
    }

    pub async fn handle_instructions(
        &self,
        req: &InstructionsReq,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(self.get_instructions(req.instruction_type.clone())?))
    }

    pub async fn handle_statement<I: Issuer>(
        &self,
        // TODO: REMOVE THIS ONCE ALL DEMOS HAVE BEEN MIGRATED TO PUBLISHED REBASE!
        statement: &CompatStatementReq,
        issuer: &I,
    ) -> Result<serde_json::Value, FlowError> {
        Ok(json!(
            self.statement(&statement.to_statement(), issuer).await?
        ))
    }
}

pub async fn handle_verify(
    req: &VCWrapper,
    resolver_opts: &Option<ResolverOpts>,
) -> Result<(), FlowError> {
    let issuer = match &req {
        VCWrapper::Jwt(r) => {
            let c = Credential::from_jwt_unsigned(&r.jwt)
                .map_err(|e| FlowError::Validation(e.to_string()))?;
            if c.issuer.is_none() {
                return Err(FlowError::Validation(
                    "No issuer found in the Credential".to_string(),
                ));
            }

            c.issuer.unwrap().get_id()
        }
        VCWrapper::Ld(r) => {
            if r.credential.issuer.is_none() {
                return Err(FlowError::Validation(
                    "No issuer found in the Credential".to_string(),
                ));
            }

            r.credential.issuer.as_ref().unwrap().get_id()
        }
    };

    let v_method = get_verification_method(&issuer, &make_resolver(resolver_opts)).await;
    if v_method.is_none() {
        return Err(FlowError::Validation(
            "Could not generate verifcation method".to_string(),
        ));
    }
    let vm = v_method.unwrap();

    let ldpo = LinkedDataProofOptions {
        verification_method: Some(URI::String(vm)),
        ..Default::default()
    };

    let res = match req {
        VCWrapper::Jwt(r) => {
            Credential::verify_jwt(
                &r.jwt,
                Some(ldpo),
                &make_resolver(resolver_opts),
                &mut context_loader()?,
            )
            .await
        }
        VCWrapper::Ld(r) => {
            r.credential
                .verify(
                    Some(ldpo),
                    &make_resolver(resolver_opts),
                    &mut context_loader()?,
                )
                .await
        }
    };

    if res.errors.is_empty() {
        Ok(())
    } else {
        let message = res.errors.join(" ");
        Err(FlowError::BadLookup(message))
    }
}
