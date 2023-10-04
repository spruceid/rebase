use crate::{
    content::delegated_attestation::{
        content::DelegatedAttestationContent,
        delegated_basic_image_attestation::DelegatedBasicImageAttestationContent,
        delegated_basic_post_attestation::DelegatedBasicPostAttestationContent,
        delegated_basic_profile_attestation::DelegatedBasicProfileAttestationContent,
        delegated_basic_tag_attestation::DelegatedBasicTagAttestationContent,
        delegated_book_review_attestation::DelegatedBookReviewAttestationContent,
        delegated_dapp_preferences_attestation::DelegatedDappPreferencesAttestationContent,
        delegated_follow_attestation::DelegatedFollowAttestationContent,
        delegated_like_attestation::DelegatedLikeAttestationContent,
        delegated_progress_book_link_attestation::DelegatedProgressBookLinkAttestationContent,
    },
    statement::attestation::statement::AttestationStatement,
    subject::ethereum::Eip155,
    types::{
        capability::recap::from_action_string,
        defs::{Proof, Statement, Subject},
        enums::{
            attestation::AttestationTypes,
            subject::{Pkh, Subjects},
        },
        error::{ProofError, StatementError},
    },
};
use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use base64::engine::Engine as _;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use siwe::{eip55, Message};
use std::str::FromStr;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DelegatedAttestationProof {
    pub attestation: AttestationStatement,
    pub attestation_signature: String,
    pub service_key: String,
    pub siwe_message: String,
    pub siwe_signature: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ParsedReCap {
    // The did:key:... address of the delegate key
    pub delegate: String,
    // The subject of the credential / signer of the SIWE message
    pub subject: Subjects,
    // the approved credential types to issue
    pub types: Vec<AttestationTypes>,
}

pub const RECAP_PREFIX: &str = "urn:recap:";

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
struct HexRecap {
    pub att: Map<String, Value>,
    // NOTE: Currently ignored.
    // #[ts(type = "Array<any>")]
    // pub prf: Vec<Value>,
}

pub fn parse_siwe_recap(siwe_recap: &str, service_key: &str) -> Result<ParsedReCap, ProofError> {
    let m = Message::from_str(siwe_recap).map_err(|e| {
        ProofError::ContentGeneration(format!("Failed to parse ReCap into Message: {}", e))
    })?;

    let delegate = m.uri.to_string();

    let address = eip55(&m.address);
    let subject = Subjects::Pkh(Pkh::Eip155(Eip155 {
        address,
        chain_id: format!("{}", m.chain_id),
    }));

    let mut r: Option<HexRecap> = None;
    let mut counter = 0;

    for s in m.resources {
        let s = s.to_string();
        if s.starts_with(RECAP_PREFIX) {
            let s = s.trim_start_matches(RECAP_PREFIX);
            if let Ok(s) = BASE64.decode(s.as_bytes()) {
                if let Ok(s) = String::from_utf8(s) {
                    if let Ok(parsed) = serde_json::from_str::<HexRecap>(&s) {
                        for (k, _) in parsed.att.clone() {
                            if k == service_key {
                                if counter > 0 {
                                    return Err(ProofError::ContentGeneration("Multiple ReCaps for the same host found, please only provide one entry per Resources for Rebase ReCaps".to_string()));
                                }
                                r = Some(parsed.clone());
                                counter += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let mut types: Vec<AttestationTypes> = Vec::new();

    match r {
        Some(hex_recap) => {
            if let Some(v) = hex_recap.att.get(service_key) {
                let attestation_map: Map<String, Value> = serde_json::from_value(v.clone())
                    .map_err(|e| {
                        ProofError::ContentGeneration(format!(
                            "Failed to deserialize service_key's contained map: {}",
                            e
                        ))
                    })?;

                // TODO: Use _ to check for caveats, if those are implemented.
                for (k, _) in attestation_map {
                    if let Some(t) = from_action_string(&k) {
                        types.push(t);
                    }
                }

                if types.is_empty() {
                    Err(ProofError::ContentGeneration(
                        "Found no attestation types in ReCap".to_string(),
                    ))
                } else {
                    Ok(ParsedReCap {
                        delegate,
                        subject,
                        types,
                    })
                }
            } else {
                Err(ProofError::ContentGeneration(
                    "Could not find service_key in Recap att property".to_string(),
                ))
            }
        }
        None => Err(ProofError::ContentGeneration(format!(
            "Could not find service_key {} in Recap resources",
            service_key
        ))),
    }
}

impl Statement for DelegatedAttestationProof {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.attestation.generate_statement()
    }
}

impl Proof<DelegatedAttestationContent> for DelegatedAttestationProof {
    fn to_content(
        &self,
        _statement: &str,
        _signature: &str,
    ) -> Result<DelegatedAttestationContent, ProofError> {
        let delegate = parse_siwe_recap(&self.siwe_message, &self.service_key)?.delegate;
        match &self.attestation {
            AttestationStatement::BasicImageAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::BasicImageAttestation(
                    DelegatedBasicImageAttestationContent {
                        id: x.subject.did()?,
                        src: x.src,
                        delegate,
                    },
                ))
            }
            AttestationStatement::BasicPostAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::BasicPostAttestation(
                    DelegatedBasicPostAttestationContent {
                        id: x.subject.did()?,
                        body: x.body,
                        title: x.title,
                        reply_to: x.reply_to,
                        delegate,
                    },
                ))
            }
            AttestationStatement::BasicProfileAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::BasicProfileAttestation(
                    DelegatedBasicProfileAttestationContent {
                        id: x.subject.did()?,
                        image: x.image,
                        username: x.username,
                        website: x.website,
                        description: x.description,
                        delegate,
                    },
                ))
            }
            AttestationStatement::BasicTagAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::BasicTagAttestation(
                    DelegatedBasicTagAttestationContent {
                        id: x.subject.did()?,
                        post: x.post,
                        users: x.users,
                        delegate,
                    },
                ))
            }
            AttestationStatement::BookReviewAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::BookReviewAttestation(
                    DelegatedBookReviewAttestationContent {
                        id: x.subject.did()?,
                        link: x.link,
                        rating: x.rating,
                        review: x.review,
                        delegate,
                        title: x.title,
                    },
                ))
            }
            AttestationStatement::DappPreferencesAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::DappPreferencesAttestation(
                    DelegatedDappPreferencesAttestationContent {
                        id: x.subject.did()?,
                        dark_mode: x.dark_mode,
                        delegate,
                    },
                ))
            }
            AttestationStatement::FollowAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::FollowAttestation(
                    DelegatedFollowAttestationContent {
                        id: x.subject.did()?,
                        target: x.target,
                        delegate,
                    },
                ))
            }
            AttestationStatement::LikeAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::LikeAttestation(
                    DelegatedLikeAttestationContent {
                        id: x.subject.did()?,
                        target: x.target,
                        delegate,
                    },
                ))
            }
            AttestationStatement::ProgressBookLinkAttestation(x) => {
                let x = x.clone();
                Ok(DelegatedAttestationContent::ProgressBookLinkAttestation(
                    DelegatedProgressBookLinkAttestationContent {
                        id: x.subject.did()?,
                        link: x.link,
                        progress: x.progress,
                        delegate,
                    },
                ))
            }
        }
    }
}
