use crate::{
    content::delegated_attestation::content::DelegatedAttestationContent,
    issuer::ed25519::Ed25519Jwk,
    proof::delegated_attestation::{parse_siwe_recap, DelegatedAttestationProof},
    statement::attestation::statement::AttestationStatement,
    types::{
        defs::{
            eip55, resolve_key, to_action, DIDKey, Flow, Instructions, Issuer, Message, Proof,
            Statement, StatementResponse, Subject,
        },
        enums::{
            attestation::Attestation,
            subject::{Pkh, Subjects},
        },
        error::FlowError,
    },
};
use async_trait::async_trait;
use did_web::DIDWeb;
use schemars::schema_for;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DelegatedAttestationFlow {
    pub service_key: String,
}

#[async_trait(?Send)]
impl Flow<DelegatedAttestationContent, AttestationStatement, DelegatedAttestationProof>
    for DelegatedAttestationFlow
{
    fn instructions(&self) -> Result<Instructions, FlowError> {
        // NOTE: These instructions are for all witnessed flows.
        Ok(Instructions {
            statement: "Fill out the presented form to create content in the form of a credential."
                .to_string(),
            statement_schema: schema_for!(AttestationStatement),
            signature: "".to_string(),
            witness: "Present the signature and the content object to the witness to have it transformed into a credential.".to_string(),
            witness_schema: schema_for!(DelegatedAttestationProof)
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &AttestationStatement,
        _issuer: &I,
    ) -> Result<StatementResponse, FlowError> {
        Ok(StatementResponse {
            statement: statement.generate_statement()?,
            delimiter: None,
        })
    }

    async fn validate_proof<I: Issuer>(
        &self,
        proof: &DelegatedAttestationProof,
        _issuer: &I,
    ) -> Result<DelegatedAttestationContent, FlowError> {
        // Check that the SIWE message is valid.
        // TODO: Use Message's own verify methods instead?
        let parsed_recap = parse_siwe_recap(&proof.siwe_message, &self.service_key)?;
        parsed_recap
            .subject
            .valid_signature(&proof.siwe_message, &proof.siwe_signature)
            .await?;

        // Create the SIWE message to use a source of information for validation.
        let m = Message::from_str(&proof.siwe_message).map_err(|e| {
            FlowError::BadLookup(format!("Failed to parse ReCap into Message: {}", e))
        })?;

        // Use SIWE's own validation to determine if the ReCap is valid from a timing perspective
        if !m.valid_now() {
            return Err(FlowError::BadLookup(
                "Capability is not valid at this time".to_string(),
            ));
        }

        // Check that the attestation type is supported by the ReCap
        let (t, _) = proof.attestation.to_statement()?;
        if !parsed_recap.types.contains(&t) {
            return Err(FlowError::BadLookup(format!(
                "ReCap did not authorize issuance of credential type: {}",
                to_action(&t)
            )));
        }

        // Check that the attestation's subject is that of the delegator
        let subject = proof.attestation.subject();
        let address = match subject {
            Subjects::Pkh(Pkh::Eip155(x)) => x.address,
            // TODO: Support other keys?
            _ => {
                return Err(FlowError::BadLookup(format!(
                    "Subject expected to be did:pkh:eip155, got: {}",
                    subject.did()?
                )))
            }
        };

        if address != eip55(&m.address) {
            return Err(FlowError::BadLookup(format!(
                "Attestation subject is {} but SIWE signer is {}",
                address,
                eip55(&m.address)
            )));
        }

        // Check that the attestation signature is valid.
        // Create the Public Key to check the signature by parsing the SIWE message.
        let full_delegate_did = m.uri.to_string();
        let split_did: Vec<String> = m
            .uri
            .to_string()
            .split('#')
            .map(|s| s.to_string())
            .collect();

        if split_did.len() != 2 {
            return Err(FlowError::BadLookup(
                "Delegate DID was not in expected format".to_string(),
            ));
        }

        let delegate_did = split_did[0].clone();
        let key_name = split_did[1].clone();

        // Create JWK from DID Url
        let delegate_jwk = if delegate_did.starts_with("did:key:") {
            let resolver = DIDKey {};
            resolve_key(&full_delegate_did, &resolver)
                .await
                .map_err(|e| FlowError::BadLookup(format!("Could not build JWK from DID: {}", e)))?
        } else if delegate_did.starts_with("did:web:") {
            let resolver = DIDWeb {};
            resolve_key(&full_delegate_did, &resolver)
                .await
                .map_err(|e| FlowError::BadLookup(format!("Could not build JWK from DID: {}", e)))?
        } else {
            return Err(FlowError::BadLookup(format!(
                "Delegate DID must be of did:web or did:key, got {}",
                delegate_did
            )));
        };

        // Generate Rebase Subject, then call valid_signature.
        let json_jwk = serde_json::to_string(&delegate_jwk).map_err(|e| {
            FlowError::BadLookup(format!("Could not serailize JWK from did resolver: {}", e))
        })?;

        Ed25519Jwk::new(&delegate_did, &json_jwk, &key_name)?
            .valid_signature(
                &proof.attestation.generate_statement()?,
                &proof.attestation_signature,
            )
            .await?;

        // Victory
        Ok(proof.to_content(&proof.generate_statement()?, &proof.attestation_signature)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        statement::attestation::basic_post_attestation::BasicPostAttestationStatement,
        test_util::util::{test_did_keypair, test_eth_did},
    };

    #[tokio::test]
    async fn test_delegated_attestation() {
        // Due to the inability to replicate things like SIWE ReCaps on the fly, this is a fairly simple test.
        let proof = DelegatedAttestationProof {
            attestation: AttestationStatement::BasicPostAttestation(BasicPostAttestationStatement {
                title: "Hello".to_string(),
                body: "World".to_string(),
                reply_to: None,
                subject: test_eth_did(),
            }),
            attestation_signature: "cf12bcc0dabf76651407cb88a72808585df23d60dcb22b0b10a14dff3da2ff54017c7991cf01b4fc4cba617d3aaa785cbb6f5c1b33f10846ed23550e5324e106".to_string(),
            service_key: "rebase:did:web:rebasedemokey.pages.dev".to_string(),
            siwe_message: "localhost:8080 wants you to sign in with your Ethereum account:\n0xdA3176d77c04632F2862B14E35bc6B4717FB5016\n\nI further authorize the stated URI to perform the following actions on my behalf: (1) 'issue': 'basic_post_attestation' for 'rebase:did:web:rebasedemokey.pages.dev'.\n\nURI: did:key:z6MkiqEVE7UdwpRncdBH5QQQ7THmd8DzuANApbmaXyXNKPSc#z6MkiqEVE7UdwpRncdBH5QQQ7THmd8DzuANApbmaXyXNKPSc\nVersion: 1\nChain ID: 1\nNonce: 6JQhF2R1wBhfF6ONV\nIssued At: 2023-09-27T17:11:32.013Z\nExpiration Time: 2123-09-27T17:11:32.014Z\nNot Before: 2022-09-27T17:11:32.013Z\nResources:\n- urn:recap:eyJhdHQiOnsicmViYXNlOmRpZDp3ZWI6cmViYXNlZGVtb2tleS5wYWdlcy5kZXYiOnsiaXNzdWUvYmFzaWNfcG9zdF9hdHRlc3RhdGlvbiI6W3t9XX19LCJwcmYiOltdfQ".to_string(),
            siwe_signature: "0xa5f8764d637cab627245b5e008b06f04c50361e34e2b19f1a940646373e7f1810385fd8d5c1501b7f0d899f95603cc4632bc9cd77454f24b5e0d64493657e6161c".to_string(),
        };

        let flow = DelegatedAttestationFlow {
            service_key: "rebase:did:web:rebasedemokey.pages.dev".to_string(),
        };

        let (_, issuer) = test_did_keypair().await.unwrap();

        flow.jwt(&proof, &issuer).await.unwrap();
    }
}
