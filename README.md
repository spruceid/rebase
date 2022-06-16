# rebase

Rebase is a library for handling the witnessing of cryptographically verifiable claims and issuance of Verifiable Credentials based on this programmatic witnessing. More specifically, Rebase simplifies the process of creating links between identity providers, or self attested claims using VCs by providing a convient wrapper around SSI. Rebase is intended for a wide variety of uses ranging from server-side "witness" serivces, to VC reading validation services, to in-browser usage via WASM. 

## Architectural Overview

The heart of the project is found in `rust/rebase/src`. The high level goal is to collect data from the end user, create a statement for the user to sign, ask for the signature from the user (in addition to some other information in some cases), and presuming the statement and the signature match, issue a credential. Some flows are simpler than others, but all follow this basic format. Other uses of Rebase also exist, like verifying existing credentials produced by the described workflow, but most exist in that context.

Rebase works by layering several abstractions over each other. At the base is the `SignerType`, which defines what cryptographic signature could be read in a claim and how it could be verified. A layer above that is the `Signer<T: SignerType>` which is struct capable of signing both plain text (in the case of a client) and a VC (in the case of an issuer). In the simplest flow, the issuer is the client, but these types of claims don't link identities, simply show the signer signed whatever is stated in the VC (in other words "self attested"). Self attested claims still can be useful in the case of things like decentralized social media posts, etc.

The next important abstraction is the `SchemaType` which is a trait that takes a simple struct, something like:
```rust
// src/witness/github.rs
pub struct Schema {
    pub gist_id: String,
    pub handle: String,
    pub key_type: SignerDID,
    pub statement: String,
    pub signature: String,
}
```
Then implements the following portion of this trait to generate the pieces of the VC from the given SchemaType:
```rust
// src/schema/schema_type
pub trait SchemaType {
    // ...
    // Return the @context contents based enum variant
    fn context(&self) -> Result<serde_json::Value, SchemaError>;

    // Returns the evidence entry for VC
    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError>;

    // TODO: Better type?
    // Returns the object used in credentialSubject
    fn subject(&self) -> Result<serde_json::Value, SchemaError>;

    // Return the types used in credential building.
    fn types(&self) -> Result<Vec<String>, SchemaError>;
}
```

The result is that the following functions are derived:
```rust
    // Return the unsigned credential using a signer type.
    async fn unsigned_credential<T: SignerType>(
        &self,
        signer_type: &T,
    ) -> Result<Credential, SchemaError> {
        // ...
    }

    // Return the complete, signed LD Proof credential
    async fn credential<T: SignerType>(
        &self,
        signer: &dyn Signer<T>,
    ) -> Result<Credential, SchemaError> {
        // ...
    }

    // Return a JWT signed credential
    async fn jwt<T: SignerType>(&self, signer: &dyn Signer<T>) -> Result<String, SchemaError> {
        // ...
    }
```

Because the `SignerType` provides one portion of the VC's construction, and the `SchemaType` provides the rest, and that the `Signer<T: SignerType>` provides the signature to a given `SchemaType`, all of these pieces can be mixed and matched. If a new `SchemaType` is implemented, it works with all existing `Signer`/`SignerType`s. If a new `Signer` is implemented, it works with all existing `SchemaType`s.

The final set of abstractions are a toolkit for building witnessing services. 

The witnessing flow looks like:

1) Gather information from the user to give data for a statement.

2) Give the user a statement to sign that describes the `SignerType` that should be used to sign the statement.

3) The user signs the statement. The user returns the statement and enough information to verify the signature. In the case of linking public profiles, this would be retrieving a public post (a tweet, a gist, etc) that contains the statement and signature, parsing them, then verifing that signature is of the statement and by the `SignerType` described in the statement. In the case of linking two keys, this would just be the two `SignerTypes` and two signatures.

4) The witness preforms the steps described above and either issues a VC or returns the error.

To make this possible, first a struct must implement the `Statement` trait in `src/witness/witness`, then when a user supplies such a struct, they are given back a statement to sign and a delimitor (if applicable) to place between the statement and the signature.

Once the user has the statement to sign, then they often have to post the combination of `format!("{}{}{}", statement, delimitor, signature)` (DNS is an exception to this rule, using a `prefix` and `format!("{}{}{}", prefix, delimitor, signature)`). Once they have posted the statement (if neccesary), they then have to provide enough information to create a struct that implements `Proof`. `Proof` must implement `Statement` to allow the witness to make sure that the statement found is the same as expected. Often, the same struct implements `Proof` and `SchemaType`. 

The final abstraction is the witness, contained in the `Generator` trait. This trait requires the user to implement a pair of functions:
```rust
// src/witness/witness
#[async_trait(?Send)]
pub trait Generator<P: Proof, S: SchemaType> {
    // From the proof structure, look up the statement and signature.
    async fn locate_post(&self, proof: &P) -> Result<String, WitnessError>;

    // From the proof structure, create a schema structure without any checks.
    fn _unchecked_to_schema(
        &self,
        proof: &P,
        statement: &str,
        signature: &str,
    ) -> Result<S, WitnessError>;
    ...
}
```
Which then derives the following functions:
```rust
    // From the proof structure, create a schema.
    async fn schema(&self, proof: &P) -> Result<S, WitnessError> {
        let post = self.locate_post(proof).await?;
        let (statement, signature) = proof.parse_post(&post).await?;
        Ok(self._unchecked_to_schema(proof, &statement, &signature)?)
    }

    // From the proof structure, create a LD credential.
    async fn credential<T: SignerType>(
        &self,
        proof: &P,
        signer: &dyn Signer<T>,
    ) -> Result<Credential, WitnessError> {
        Ok(self.schema(proof).await?.credential(signer).await?)
    }

    // From the proof structure, create a JWT.
    async fn jwt<T: SignerType>(
        &self,
        proof: &P,
        signer: &dyn Signer<T>,
    ) -> Result<String, WitnessError> {
        Ok(self.schema(proof).await?.jwt(signer).await?)
    }
```
This allows a witness to be as simple as a struct that implements `Generator` to recieve a valid `Proof` and return a `Schema`, a `Credential`, or a JWT `String` depending on what is requested. The derived `schema` function only allows the creation of credentials if they pass the parsing stage.

In the case of `DNS`, the `Generator` is an empty struct, in the case of `Twitter`, the `Generator` has an `api_key` field. Any required information for the post retrieval process can be specified in a struct, then that struct made to implement `Generator`.

To maximize the ability to mix and match credentials several helper structs can be found in `src/witness`, specifically `ProofTypes`, `StatementTypes` and `SignerTypes`, these are two enums which encompass all supported `Proof`s and `SignerType`s, then implement `Proof` and `SignerType` on the enum by calling their inner, concrete representation.

Similiarly, in `src/signer/signer` there is a `DID` enum which captures all the supported `SignerType`s in a generic struct. To implement `SignerType`, it's required to have the following function implemented:
```rust
    fn new(t: &DID) -> Result<Self, SignerError>;
```

This allows us to capture all valid `SignerType`s in `src/signer/signer` but not have circular dependencies, and also allows for easy converstion back and forth between `DID` and `SignerType`.

The useful result of these enum abstractions is the ability to create a universal generator, available for import from `src/witness/generator`. Given a supported `Proof` (i.e. those listed in `ProofTypes`) and a supported `SignerType` (i.e. those listed in `SignerTypes`), the generator can validate a claim and produce a VC.

Statements work similiarly with `StatementTypes` and `SignerTypes`. Thus, the calling application doesn't even have to be aware of all the possible claims it can validate -- seen in the example worker.

## Examples

The `demo` directory includes a Cloudflare Worker that acts as a server-side witness (`demo/witness`) and a front-end UI for interacting with the witness (`demo/dapp`). Installation and usage instructions are found in those respective directories, but the high-level overview is given here. 

The Cloudflare Worker acts a proof-of-concept that Rebase can be packaged for WASM environments, including the browser. Otherwise, it essentially functions as a tiny HTTP server. It contains 2 routes, `/statement`, where client is expected to post a struct that implements `Statement` then recieves the generated statement from the witness and `/witness` where a struct that implements `Proof` is posted, and the witness uses it's generator to produce a VC (assuming all the details check out).

The UI is a thin client that simply gathers the information required to generate the statement, interacts with browser extensions to get the user to sign the statement, informs the user where to post the statement (if neccesary), then gathers the information on the location of the post (again, if neccesary), returns it to the witness for a VC, then displays the VC and allows the user to download it.

## Implementing New Features

### Implementing New Signers
To implement a new `Signer`, the first step is to implement a `SignerType`. In most cases, `SignerType` will refer to a public key and `Signer<SignerType>` will refer to a private key corresponding to the `SignerType`. 

To implement a `SignerType` you must implement the following trait:
```rust
// src/signer/signer
#[async_trait(?Send)]
pub trait SignerType
where
    Self: Sized,
{
    fn name(&self) -> String;

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SignerError>;

    fn did_id(&self) -> Result<String, SignerError>;

    fn new(t: &DID) -> Result<Self, SignerError>;

    fn did(&self) -> DID;
}
```

The implementation for `ed25519` looks like:
```rust
// src/signer/ed25519
#[derive(Clone)]
pub enum Ed25519 {
	// TODO: Change name?
	DIDWebJWK(Option<String>),
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
```

Once this has been implemented, the next step is to add it's `did` representation to `src/signer/signer`'s `DID` enum, which as of time of writing looks like:
```rust
// src/signer/signer
#[derive(Clone, Deserialize, Serialize)]
pub struct EIP115 {
    pub address: String,
    pub chain_id: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum PKH {
    #[serde(rename = "eip115")]
    EIP115(Option<EIP115>),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum DID {
    #[serde(rename = "pkh")]
    PKH(PKH),
    // NOTE: Currently only supports Ed25519 keys for signing
    // Could change did::web to an enum if desired.
    #[serde(rename = "web")]
    Web(Option<String>),
}
```

Additional slots can be added at any level of the enum safely. Once the `DID` representation is complete, to use the new SignerType in witness flows, you will need to add it to `src/witness/signer_type`, both in the `SignerTypes` enum:
```rust
// src/witness/signer_type
pub enum SignerTypes {
    Ed25519(Ed25519),
    Ethereum(Ethereum),
}
```

In the `impl SignerType` for `SignerTypes`, and the `statement_id` function for `SignerTypes`. The `statement_id` function is used for putting the indentifier in public claims, and often the `did_id` is not desired, so it usually parses the `did_id` into something simpler. This _should_ be made part of `SignerType` trait, and may be moved there in the future.

At that point a new `SignerType` is implemented, and implementing a `Signer` is going to be a bit easier. The `Signer` for `ed25519` is implemented like so:
```rust
// src/signer/ed25519
pub struct Ed25519DidWebJwk {
	pub id: String,
	pub key: JWK,
	pub key_name: String,
	signer_type: Ed25519,
}

// ...

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

		Ok(Some(OneOrMany::One(
			vc.generate_proof(&self.key, &lpdo, &DIDWeb).await?,
		)))
	}

	fn id(&self) -> String {
		self.id.clone()
	}

	fn signer_type(&self) -> Ed25519 {
		self.signer_type.clone()
	}
}
```

As can be seen, the `SignerType` for a given `Signer` is often going to be concrete at the `impl Signer<...>` level. The key here is to be able to provide a `proof` entry for the VC and to be able to `sign` bytes and `sign_vc` for VCs. If a `Signer` implements `sign`, it can be used to sign claims as a client, if it implements `sign_vc`, it can be used to author VCs as a witness.

It is not neccesary to implement `Signer` if the expectation is that a particular `SignerType` will only be used by the client. As of writing, `ethereum` only implements `SignerType` and cannot be used to issue VCs, only to sign claims that a witness can validate.

### Implementing New Schemas

It is very simple to implement a new schema on it's own (implementing the witness flow is a seperate concern covered shortly). The most basic `Schema` supported by Rebase is the `basic_post` credential. It is not expected to be witnessed, but rather self-produced and self-signed, so it is a very simple credential. It looks like:
```rust
// src/schema/basic_post
#[derive(Deserialize, Serialize)]
pub struct BasicPost {
    pub title: String,
    pub body: String,
    pub subject_id: String,
}

impl SchemaType for BasicPost {
    fn context(&self) -> Result<serde_json::Value, SchemaError> {
        // TODO: MAKE THESE URLS MORE ACCURATE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
              "title": "https://schema.org/name",
              "body": "https://schema.org/articleBody",
              "BasicPost": "https://schema.org/BlogPosting"
          },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, SchemaError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicPost".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, SchemaError> {
        Ok(json!({
            "id": self.subject_id,
            "title": self.title,
            "body": self.body,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, SchemaError> {
        Ok(None)
    }
}
```

The required functions return portions of the VC to be constructed (the other portions are supplied by the given `SignerType`) relating to the `@context`, `types`, `credentialSubject`, and `evidence` entries. Once these are defined, the schema can be mixed and matched with all implemented `SignerType`s.

### Implementing New Witness Flows

Witness flows are built on top of schemas. They are more complex because they require defining a struct that implements `Statement` and a struct (sometimes the same as first) that implements `Proof` and `Statement`.

Once those two traits are implemented, a `Generator<Proof, Schema>` must also be implemented. One of the simplest is DNS:

```rust
// src/witness/dns
#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub domain: String,
    pub prefix: String,
    pub key_type: SignerDID,
}

impl Statement for Claim {
    fn signer_type(&self) -> Result<SignerTypes, SignerError> {
        SignerTypes::new(&self.key_type)
    }

    fn generate_statement(&self) -> Result<String, WitnessError> {
        let signer_type = self.signer_type()?;

        Ok(format!(
            "{} is linked to {}",
            self.domain,
            signer_type.statement_id()?
        ))
    }

    fn delimitor(&self) -> String {
        "=".to_string()
    }
}

impl Proof for Claim {}

pub struct Schema {
    pub domain: String,
    pub key_type: SignerDID,
}

impl SchemaType for Schema {
    // ...
}
```
Then the DNS generator is implemented like so:
```rust
impl Generator<Claim, Schema> for ClaimGenerator {
    async fn locate_post(&self, proof: &Claim) -> Result<String, WitnessError> {
        let client = reqwest::Client::new();
        let request_url = format!(
            "https://cloudflare-dns.com/dns-query?name={}&type=txt",
            proof.domain
        );

        let res: DnsResponse = client
            .get(Url::parse(&request_url).map_err(|e| WitnessError::BadLookup(e.to_string()))?)
            .header("accept", "application/dns-json")
            .send()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| WitnessError::BadLookup(e.to_string()))?;

        let mut sig = String::new();
        for answer in res.answer {
            let mut trimmed_signature: &str = &answer.data;
            if trimmed_signature.starts_with('"') && trimmed_signature.ends_with('"') {
                trimmed_signature = &answer.data[1..answer.data.len() - 1];
            }
            if trimmed_signature.starts_with(&proof.prefix) {
                sig = trimmed_signature.to_owned();
                break;
            }
        }

        // NOTE: We intercept the post and change it to match the <statement>=<signature>
        // style format.
        Ok(format!("{}={}", proof.generate_statement()?, sig))
    }

    fn _unchecked_to_schema(
        &self,
        proof: &Claim,
        _statement: &str,
        _signature: &str,
    ) -> Result<Schema, WitnessError> {
        Ok(Schema {
            domain: proof.domain.clone(),
            key_type: proof.key_type.clone(),
        })
    }
}
```

A more complex generator is found in `src/witness/twitter` where an `api_key` is used to make the look up. Once the `Generator` is implemented, it can be added to the `WitnessGenerator` in `src/witness/generator` and the generator will then support the new witness flow with no change to the calling applications.

## Current Features

Current Schemas Defined:
* basic_post (unwitnessed)
* basic_profile (unwitnessed)
* dns
* github
* twitter
* two-key linking

Current Witness flows:
* dns
* github
* twitter
* two-key linking

Current SignerTypes:
* ethereum
* ed25519

Current `Signer<SignerType>`s:
* `Ed25519DidWebJWK<Ed25519>`

