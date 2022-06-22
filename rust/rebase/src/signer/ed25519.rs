use crate::signer::signer::{Signer, SignerError, SignerType, DID as SignerDID};
use async_trait::async_trait;
use did_web::DIDWeb;
use ed25519_dalek::ed25519::signature::Signature;
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer as InnerSigner, Verifier};
use hex;
use serde::{Deserialize, Serialize};
use ssi::{
	jwk::{Base64urlUInt, Params, JWK},
	one_or_many::OneOrMany,
	vc::{Credential, LinkedDataProofOptions, Proof, URI},
};
use url::Url;

#[derive(Clone)]
pub enum Ed25519 {
	// TODO: Change name?
	DIDWebJWK(Option<String>),
}

pub struct Ed25519DidWebJwk {
	pub id: String,
	pub key: JWK,
	pub key_name: String,
	signer_type: Ed25519,
}

impl Ed25519DidWebJwk {
	pub async fn new(id: &str, key: &str, key_name: &str) -> Result<Self, SignerError> {
		// TODO: Validate the ID is a valid did?
		let key: JWK = serde_json::from_str(key).map_err(|e| SignerError::InvalidSignerOpts {
			signer_type: "ed25519 did jwk".to_owned(),
			reason: format!("could not deserailize JWK: {}", e),
		})?;
		let signer_type = Ed25519::new(&SignerDID::Web(Some(id.to_owned())))?;
		Ok(Ed25519DidWebJwk {
			id: id.to_owned(),
			key,
			key_name: key_name.to_owned(),
			signer_type,
		})
	}
}

#[async_trait(?Send)]
impl Signer<Ed25519> for Ed25519DidWebJwk {
	async fn sign(&self, plain_text: &str) -> Result<String, SignerError> {
		match &self.key.params {
			Params::OKP(o) => match &o.private_key {
				Some(key) => {
					let keypair = Keypair {
						secret: SecretKey::from_bytes(&key.0).map_err(|e| {
							SignerError::Sign(format!(
								"could not generate secret key: {}",
								e.to_string()
							))
						})?,
						public: PublicKey::from_bytes(&o.public_key.0).map_err(|e| {
							SignerError::Sign(format!(
								"could not generate public key: {}",
								e.to_string()
							))
						})?,
					};

					let sig = keypair.sign(&plain_text.as_bytes());

					Ok(hex::encode(sig.to_bytes()))
				}
				_ => Err(SignerError::Sign(
					"could not recover private key from jwk".to_string(),
				)),
			},
			_ => Err(SignerError::Sign(
				"could not recover private key from jwk".to_string(),
			)),
		}
	}

	async fn sign_vc(&self, vc: &mut Credential) -> Result<(), SignerError> {
		vc.proof = self.proof(vc).await?;
		Ok(())
	}

	async fn generate_jwt(&self, vc: &Credential) -> Result<String, SignerError> {
		Ok(vc
			.generate_jwt(
				Some(&self.key),
				&LinkedDataProofOptions {
					checks: None,
					created: None,
					eip712_domain: None,
					type_: None,
					verification_method: Some(URI::String(format!(
						"{}#{}",
						self.signer_type.did_id()?,
						self.key_name
					))),
					..Default::default()
				},
				&DIDWeb,
			)
			.await?)
	}

	async fn proof(&self, vc: &Credential) -> Result<Option<OneOrMany<Proof>>, SignerError> {
		let lpdo = match self.signer_type {
			Ed25519::DIDWebJWK(_) => LinkedDataProofOptions {
				verification_method: Some(URI::String(format!(
					"{}#{}",
					self.signer_type.did_id()?,
					self.key_name
				))),
				..Default::default()
			},
		};

		let mut context_loader = ssi::jsonld::ContextLoader::default();
		Ok(Some(OneOrMany::One(
			vc.generate_proof(&self.key, &lpdo, &DIDWeb, &mut context_loader).await?,
		)))
	}

	fn id(&self) -> String {
		self.id.clone()
	}

	fn signer_type(&self) -> Ed25519 {
		self.signer_type.clone()
	}
}

#[async_trait(?Send)]
impl SignerType for Ed25519 {
	fn new(t: &SignerDID) -> Result<Self, SignerError> {
		match t {
			SignerDID::Web(o) => Ok(Ed25519::DIDWebJWK(o.clone())),
			_ => Err(SignerError::InvalidSignerOpts {
				signer_type: t.to_string(),
				reason: "expected ed25519 signer type".to_string(),
			}),
		}
	}

	fn did(&self) -> SignerDID {
		match self {
			Ed25519::DIDWebJWK(o) => SignerDID::Web(o.clone()),
		}
	}

	fn name(&self) -> String {
		match self {
			Ed25519::DIDWebJWK(_) => "Ed25519 Web Key".to_string(),
		}
	}

	fn did_id(&self) -> Result<String, SignerError> {
		match self {
			Ed25519::DIDWebJWK(Some(s)) => Ok(s.to_owned()),
			_ => Err(SignerError::InvalidId {
				signer_type: self.name(),
				reason: "no id set or incorrect id type".to_string(),
			}),
		}
	}

	async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError> {
		let sig = Signature::from_bytes(&hex::decode(signature).map_err(|e| {
			SignerError::InvalidSignature {
				signer_type: self.name(),
				reason: e.to_string(),
			}
		})?)
		.map_err(|e| SignerError::InvalidSignature {
			signer_type: self.name(),
			reason: e.to_string(),
		})?;

		let stmt = statement.as_bytes();
		let pubkey = self.pubkey().await?;

		pubkey
			.verify(&stmt, &sig)
			.map_err(|e| SignerError::InvalidSignature {
				signer_type: self.name(),
				reason: e.to_string(),
			})
	}
}

impl Ed25519 {
	fn pubkey_err(&self, e: String) -> SignerError {
		SignerError::InvalidId {
			signer_type: self.name(),
			reason: format!("failed to retrieve did web key: {}", e.to_string()),
		}
	}

	async fn pubkey(&self) -> Result<PublicKey, SignerError> {
		match self {
			Ed25519::DIDWebJWK(_) => {
				// TODO: Handle ports and escaping.
				match self.did_id()?.strip_prefix("did:web:") {
					Some(u) => {
						let client = reqwest::Client::new();
						let request_url = format!("https://{}/.well-known/did.json", u);
						let res: Did = client
							.get(
								Url::parse(&request_url)
									.map_err(|e| self.pubkey_err(e.to_string()))?,
							)
							.send()
							.await
							.map_err(|e| {
								self.pubkey_err(format!(
									"failed to retrieve did web key: {}",
									e.to_string()
								))
							})?
							.json()
							.await
							.map_err(|e| self.pubkey_err(e.to_string()))?;

						if res.verification_method.len() < 1 {
							return Err(self
								.pubkey_err("no verifications found in did document".to_string()));
						};

						let b = Base64urlUInt::try_from(res.verification_method[0].key.x.clone())
							.map_err(|e| {
							self.pubkey_err(format!(
								"failed to decode public key: {}",
								e.to_string()
							))
						})?;

						Ok(PublicKey::from_bytes(&b.0).map_err(|e| {
							self.pubkey_err(format!(
								"failed to create from bytes: {}",
								e.to_string()
							))
						})?)
					}
					None => Err(SignerError::InvalidId {
						signer_type: self.name(),
						reason: format!("Unexpected did web format: {}", self.did_id()?),
					}),
				}
			}
		}
	}
}

#[derive(Deserialize, Serialize)]
pub struct Did {
	#[serde(rename = "@context")]
	// pub context: Vec<Context>,
	pub context: String,
	pub id: String,
	#[serde(rename = "verificationMethod")]
	pub verification_method: Vec<VerificationMethod>,
	pub authentication: Vec<String>,
	#[serde(rename = "assertionMethod")]
	pub assertion_method: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationMethod {
	pub id: String,
	#[serde(rename = "type")]
	pub _type: String,
	pub controller: String,
	#[serde(rename = "publicKeyJwk")]
	pub key: VerificationPubKey,
}

#[derive(Deserialize, Serialize)]
pub struct VerificationPubKey {
	pub kty: String,
	pub crv: String,
	pub x: String,
}

#[derive(Deserialize, Serialize)]
pub struct ContextKey {
	#[serde(rename = "Ed25519VerificationKey2018")]
	pub verification_key: String,
	#[serde(rename = "publicKeyJwk")]
	pub public_key: ContextPubKey,
}

#[derive(Deserialize, Serialize)]
pub struct ContextPubKey {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "@type")]
	pub _type: String,
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Context {
	String(String),
	Struct(ContextKey),
}
