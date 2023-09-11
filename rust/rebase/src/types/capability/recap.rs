use crate::types::enums::attestation::AttestationTypes;
pub use siwe_recap::Capability as RecapCapability;
use strum::IntoEnumIterator;

pub fn to_action(attestation_type: &AttestationTypes) -> String {
    match attestation_type {
        AttestationTypes::BasicImageAttestation => "issue/basic_image_attestation".to_string(),
        AttestationTypes::BasicPostAttestation => "issue/basic_post_attestation".to_string(),
        AttestationTypes::BasicProfileAttestation => "issue/basic_profile_attestation".to_string(),
        AttestationTypes::BasicTagAttestation => "issue/basic_tag_attestation".to_string(),
        AttestationTypes::BookReviewAttestation => "issue/book_review_attestation".to_string(),
        AttestationTypes::DappPreferencesAttestation => {
            "issue/dapp_preferences_attestation".to_string()
        }
        AttestationTypes::FollowAttestation => "issue/follow_attestation".to_string(),
        AttestationTypes::LikeAttestation => "issue/like_attestation".to_string(),
        AttestationTypes::ProgressBookLinkAttestation => {
            "issue/progress_book_link_attestation".to_string()
        }
    }
}

pub fn from_action_string(action_string: &str) -> Option<AttestationTypes> {
    AttestationTypes::iter().find(|t| to_action(t) == action_string)
}
