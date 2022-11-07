# Rebase -- A Library that makes Verifiable Credential Issuance Easy!

Rebase offers an extensible, opinionated, trait-based approach to building credentialing systems on top of the foundation of [ssi](https://github.com/spruceid/ssi).

Rebase is a library that enables users and projects to easily issue their own Verifiable Credentials (VCs). It supports two types of credential flows: [self-signed credentials](#credential-1-basic-post-self-attested) and [witnessed credentials](#credential-2-github-verification-witnessed). It provides points of extension to add new credentialing flows of either type and new cryptographic subjects/issuers. All subjects, issuers, and credentialing flows can be mixed and matched. The goal of Rebase is to provide these tools for both client and server side usage. For this reason, WASM is a first class use-case. 

The output of Rebase libraries are [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/), both [Linked Data Proof](https://www.w3.org/standards/semanticweb/data) and [JWT](https://www.rfc-editor.org/rfc/rfc7519.html) formats are supported. Rebase uses [Decentralized Identifiers (DIDs)](https://www.w3.org/TR/did-core/) to represent the subjects and issuers of these credentials. If this all sounds very abstract, it will get much more concrete shortly.

## Usage

If you want a client to interact with an existing witness, or if you want to create your own witness service, take a look at the [Witness SDK](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk), built on top of this library. Otherwise, the tour of Rebase section describes the internal architecture, how to extend Rebase with new credentials/subjects/issuers, and how to directly use the basic building blocks.

## Current Features

Current credentials supported:
* basic_post (self-issued)
* basic_profile (self-issued)
* dns
* github
* reddit
* soundcloud
* twitter
* two-key linking

Current Witness flows:
* dns
* email
* github
* reddit
* soundcloud
* twitter
* two-key linking

Current Subjects:
* ethereum
* ed25519
* solana

Current Issuers:
* ed25519 (via did:web)
## A Tour of Rebase
### Motivation

The goal of the Rebase library is to provide tools that enable the calling application to easily create VCs, no matter whether it's a client self-issuing a credential or it's a server-side witness service transforming an assertation into a credential. 

To understand what Rebase provides, let's take a quick look at two examples of credentials it supports and why these are useful.

#### Credential #1: Basic Post (Self Attested)
```json
{
  "@context":[
    "https://www.w3.org/2018/credentials/v1",
    {
      "BasicPost":{
        "@context":{
          "body":"https://schema.org/articleBody",
          "title":"https://schema.org/name"
        },
        "@id":"https://schema.org/BasicPost"
      },
      "BasicPostCredential":"https://example.com/BasicPostCredential"
    }
  ],
  "id":"urn:uuid:aad8b773-51a6-432a-aa18-1f5016a97453",
  "type":[
    "VerifiableCredential",
    "BasicPost"
  ],
  "credentialSubject":{
    "id":"did:pkh:solana:4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ:RaNd...0m",
    "type":[
      "BasicPost"
    ],
    "body":"Dogs",
    "title":"Things I like"
  },
  "issuer":"did:pkh:solana:4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ:RaNd...0m",
  "issuanceDate":"2022-08-26T22:52:24.336Z",
  "proof":{
    "type":"Ed25519Signature2018",
    "proofPurpose":"assertionMethod",
    "verificationMethod":"did:pkh:solana:4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ:RaNd...0m",
    "created":"2022-08-26T22:52:24.341Z",
    "jws":"R4nD0m...s1G"
  }
}
```

#### Credential #2: GitHub Verification (Witnessed)
```json
{
  "@context": [
    "https://www.w3.org/2018/credentials/v1",
    {
      "GitHubVerification": "https://example.com/GitHubVerification",
      "GitHubVerificationMessage": {
        "@context": {
          "@protected": true,
          "@version": 1.1,
          "gistId": "https://example.com/gistId",
          "handle": "https://example.com/handle",
          "timestamp": {
            "@id": "https://example.com/timestamp",
            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
          }
        },
        "@id": "https://example.com/GitHubVerificationMessage"
      },
      "sameAs": "http://schema.org/sameAs"
    }
  ],
  "id": "urn:uuid:7917c129-00e0-403b-aa2e-d8f411d84c6f",
  "type": [
    "VerifiableCredential",
    "GitHubVerification"
  ],
  "credentialSubject": {
    "id": "did:pkh:solana:4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ:RaNd...0m",
    "sameAs": "https://github.com/some_handle"
  },
  "issuer": "did:web:rebase-xyz-did.pages.dev",
  "issuanceDate": "2022-05-31T17:09:56.116Z",
  "proof": {
    "type": "Ed25519Signature2018",
    "proofPurpose": "assertionMethod",
    "verificationMethod": "did:web:rebase-xyz-did.pages.dev#controller",
    "created": "2022-05-31T17:09:56.116Z",
    "jws": "jw5n0ns3n53..4nd-m0r3-5tuff"
  },
  "evidence": {
    "type": [
      "GitHubVerificationMessage"
    ],
    "handle": "some_handle",
    "gistId": "an4lph4num3r1cg1st1d",
    "timestamp": "2022-05-31T17:09:56.116Z"
  }
}
```

> NOTE: The signatures, IDs, etc are not valid. If you want to see an actual example of a credential, feel free to checkout the [example credential faucet](https://rebase.pages.dev) and generate your own. The GitHub verification will match exactly.

Both of these credentials belong to the same user, the owner of the `did:pkh:solana:4sGjMW1sUnHzSxGspuhpqLDx6wiyjNtZ:RaNd...0m` key pair (this is a fictional Solana wallet, which is in turn a [ed25519 key pair](https://ed25519.cr.yp.to/), but Rebase currently support several key pair types and can be expanded). 

The first is a post structure that could be displayed on a social media site. It's self-issued by the user of the `RaNd..0m` key and simply proves that the issuer endorses the text found in the post -- in this case, that they like dogs. Because of the VC format of this post, it is provable that the owner of the `R4nd..0m` key made this post. If a social media protocol was built out of VCs like this, it could support multiple reader apps, user-owned export of user-data, and interoperability in general. 

The second represents shared ownership the `RaNd..0m` key and the user with the username `some_handle` on GitHub. It is not self-issued, but instead issued by a `witness` -- a type of credential issuing service which Rebase supports as a primary use-case. In this example, the issuer uses a [did:web](https://w3c-ccg.github.io/did-method-web/) key instead of an [did:pkh](https://github.com/w3c-ccg/did-pkh/blob/main/did-pkh-method-draft.md) instance (which is what `R4Nd..0m` is encoded as, though at their core, they are both `ed25519` key pairs). These are currently the two types of `DID`s supported as issuers and subjects, but it could be expanded in the future. 

The flow for creating witnessed credentials is detailed in this document, but the high level overview is that the witness provides a statement attesting that `some_handle` on GitHub is linked to the `RaNd..0m` key, the owner of the `RaNd..0m` key uses it to sign the statement, then posts the statement and signature as a gist on `some_handle`'s GitHub account. The user then informs the witness of the id of that gist, the witness looks up the gist, parses the statement and signature out, verifies the handle of the owner of the gist matches the handle in the attestation and that the signature is result of the key in the attestation signing the attestation. If all of this is true, the witness issues the above credential, and includes the details of the gist look up in the `evidence` section.

With both of these credentials, a program or user could start to associate the data found in them. In this case, we've discovered the owner of "some_handle" on GitHub likes dogs via the pair of credentials which share the subject of the `R4Nd..0m` key pair. Rebase enables these sort of associations and more, allowing for identity systems and social media implementations that provide significantly more user control than currently is available.

## A Quick Look at the Anatomy of a Credential

More detail is found in the [Verifiable Credential specification](https://www.w3.org/TR/vc-data-model/), documentation on [Linked Data](https://www.w3.org/TR/ldp/), and the [DID specification](https://www.w3.org/TR/did-core/), but a quick overview of the top level properties of these credentials is as follows:

`@context`: This is used for the LD proofs to allow the document to conform to Linked Data conventions, defining all properties found in the `credentialSubject` and `evidence` sections.

`id`: A UUID meant to uniquely refer to the entire credential structure.

`type`: An array of "types" that are backed by definition in the `@context` section, encompasing the container structures which make up the `evidence` and `credentialSubject` sections.

`credentialSubject`: What the credential is about. Almost always includes the user's DID reperesentation of the key pair that is the topic of the credential, in addition to other information. In the case of the first credential, it also includes the Post's Subject and Body text.

`issuer`: The DID of the issuing key pair. In the first example, it's the same as the DID found in the `credentialSubject`, thus a self-issued credential. In the second example, it's the witness' key pair.

`issuanceDate`: An ISO 8601 timestamp denoting when the credential was created, generated by the issuer at time of issuance.

`proof`: The cryptographic signature over the contents of the VC. Described in more detail in the [VC Spec](https://www.w3.org/TR/vc-data-model/#proofs-signatures).

`evidence`: Not present in the self-issued credential. Information used by a witness to validate the assertion displayed in the `credentialSubject`.

Next, we will see how each of these sections are generated.
## Overview of the Rebase Library's structure.

Rebase's `src` folder is further divided into 8 sub-folders which interact to allow calling applications to issue credentials seen in the previous section. These sub-folders are:
* `content`
* `flow`
* `issuer`
* `proof`
* `statement`
* `subject`
* `test_util`
* `types`

As one might expect, `test_util` only contains code relevant for the automated test cases. Similarly, `types` contains a list of type definitions without concrete implementations, as well as all error varients used by the rest of the codebase. The rest of the folders each correspond to a trait defined in `src/types/types.rs` and the files inside describe what credential implements that trait. These folders form a dependency chain depending on the type of credential issued.

Self-Issued Credential Code Usage:
```
// -> = read: "depends on"
Content -> Issuer -> Subject
```

Witnessed Credential Code Usage:
```
// -> = read: "depends on"
Flow -> Proof 
Flow -> Statement
Flow -> Issuer
Proof -> Content -> Issuer -> Subject
Statement -> Subject
```

To best understand this we'll work through the most primative section (`Subject`) to the most built-up (`Flow`), detouring briefly after `Content` to look at the complete form of the [example self-issued credential](#credential-1-basic-post-self-attested), and concluding with a look at the [example witnessed credential](#credential-2-github-verification-witnessed). By the end, the process of adding additional Subjects, Issuers, Contents, and Flows should be clear.
### Subject

A subject defines what cryptographic key the credential describes something about. In practice, it correlates to the public key of a key pair. Subjects are described in terms of [DID](https://www.w3.org/TR/did-core/)s to enable them to be encoded into VCs and take advantage of [ssi](https://github.com/spruceid/ssi). Some supported subjects include Ethereum and Solana wallets and [did:web](https://w3c-ccg.github.io/did-method-web/) ed25519 keys.

A subject must implement the following trait

```rust
// src/types/types.rs
#[async_trait(?Send)]
pub trait Subject
where
    Self: Sized,
{
    fn did(&self) -> Result<String, SubjectError>;

    fn display_id(&self) -> Result<String, SubjectError>;

    async fn valid_signature(&self, statement: &str, signature: &str) -> Result<(), SubjectError>;
}
```

`did` returns the [DID](https://www.w3.org/TR/did-core/#did-subject) subject (or ID) of the subject, at time of writing, Rebase supports a pair of `did:pkh` schemes and one `did:web` scheme.

`display_id` returns a human readable ID useful for error messages and plain text assertions. Ethereum, for example, returns the `0x1234...` format, `did:web` returns the URL of the key's host.

`valid_signature` determines if the signature is the statement signed by the subject.

For this reason, Subjects are best though of as public keys of key pairs.

The result of the `Subject`'s `did` function is seen in `credential_1.credentialSubject.id` and `credential_2.credentialSubject.id`.
### Issuer

An issuer defines the cryptographic key that produces the VC. In the case of self-issued credentials, it can be the same as the subject. In the case of witnessed credentials, it will often be different. In practice, the issuer correlates to both pieces of the public-private key pair. This is emphasized in the fact that the `Issuer` trait must implement the `Subject` trait.

The whole trait definition looks like:

```rust
#[async_trait(?Send)]
pub trait Issuer
where
    Self: Subject,
{
    // sign takes plain text and returns the corresponding signature
    async fn sign(&self, plain_text: &str) -> Result<String, IssuerError>;
    // sign_vc takes a mutable reference to an incomplete VC and signs it.
    async fn sign_vc(&self, vc: &mut Credential) -> Result<(), IssuerError>;
    // generate_jwt takes a VC and returns it's formatted as a JWT:
    async fn generate_jwt(&self, vc: &Credential) -> Result<String, IssuerError>;
    // proof returns the linked data proof options for a given issuer type
    async fn proof(
        &self,
        credential: &Credential,
    ) -> Result<Option<OneOrMany<LDProof>>, IssuerError>;
}
```

`sign` is a plain text signature with the private key. This enables several useful things, including using an issuer as a client of a witnessed flow to sign a statement or creating challenges as a witness in a witnessed flow.

`sign_vc` takes an unsigned credential and mutates it into a signed credential. This mostly involves massaging data into [ssi](https://github.com/spruceid/ssi).

`generate_jwt` takes an unsigned credential and returns a valid JWT representation of the VC. This is what is used in the [Rebase example faucet](https://rebase.pages.dev/).

`proof` returns a linked data proof for use in the output credentials' "proof" section. This too is mostly about translating the representation at the boundry into one that [ssi](https://github.com/spruceid/ssi) can use.

In addition to the 1:1 mapping of `proof` function to `proof` section, the result of the Issuer's underlying `did` function is seen in `credential_1.issuer` and `credential_2.issuer`.

### Content

Content represents what the credential is actually about. In the case of self-issued credentials, this is entirely informational to the end user and has no deeper meaning than a self-attestation. In the case of witnessed credentials, it is usually an account the user wishes to link to the subject.

The Content Trait looks like:
```rust
#[async_trait(?Send)]
pub trait Content {
    // NOTE: IMPLEMENTED VIA COMPOSITION
    async fn unsigned_credential<T: Subject>(
        &self,
        subject: &T,
    ) -> Result<Credential, ContentError> {
        /* NOTE: THIS IS AUTOMATICALLY IMPLEMENTED! */
    }

    // NOTE: IMPLEMENTED VIA COMPOSITION
    async fn credential<T: Issuer>(&self, issuer: &T) -> Result<Credential, ContentError> {
        /* NOTE: THIS IS AUTOMATICALLY IMPLEMENTED! */
    }

    // NOTE: IMPLEMENTED VIA COMPOSITION
    async fn jwt<T: Issuer>(&self, issuer: &T) -> Result<String, ContentError> {
        /* NOTE: THIS IS AUTOMATICALLY IMPLEMENTED! */
    }

    fn context(&self) -> Result<serde_json::Value, ContentError>;

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError>;

    fn subject(&self) -> Result<serde_json::Value, ContentError>;

    fn types(&self) -> Result<Vec<String>, ContentError>;
}
```

The implementor of `Content` only has to implement the bottom four functions, the complete creation of credentials can then be infered (assuming the presence of an issuer). Each of these sections map 1:1 to sections in the example credentials. The implementation of Content for the `basic_post` credential is:

```rust
#[derive(Deserialize, Serialize)]
pub struct BasicPost {
    pub title: String,
    pub body: String,
    pub subject_id: String,
}

impl Content for BasicPost {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "BasicPostCredential": "https://example.com/BasicPostCredential",
                "BasicPost": {
                    "@id": "https://schema.org/BasicPost",
                    "@context": {
                        "title": "https://schema.org/name",
                        "body": "https://schema.org/articleBody",
                    }
                }
            },
        ]))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_string(),
            "BasicPost".to_string(),
        ])
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject_id,
            "type": ["BasicPost"],
            "title": self.title,
            "body": self.body,
        }))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        Ok(None)
    }
}
```

In short, the result of `subject` matches the `BasicPost` struct, `context` describes the structure of `subject`, `types` describes the container types in `context`, and evidence returns `None`, because there is no evidence for a self-issued credential. All the credential means is that the subject issued it.

At this point, we've now completely described how all of the sections of the first example credential was created. We'll look at the GitHub credential's implementation of `Content` later, it requires more context.

### Implementing Content for Self Issued Credentials, Issuers, and Subjects

Implementing new subjects and issuers should seem relatively straight forward. If the key pair has representation in did:pkh, or one is willing to use did:web, matching the existing implementations is relatively straight forward. Otherwise, assuming one can verify a statement and signature from a public key, it can have `Subject` implemented on it, and if it has the ability to both sign bytes and be formatted in such a way that [ssi](https://github.com/spruceid/ssi) accepts it, then it can sign VCs and it can be `Issuer` as well.

There are likely use-cases where a key pair is a good Subject but poor issuer. This is one of the use cases for witnessed credentials.

It should be clear that all you have to do to implement a new self-issued credential is to create a struct, then when implementing `Content` for it, you describe your structure in `context`, then you describe your containers in `types`, then you transpose the data from your struct in `subject` (while referencing the `container` in `types`). Once the data has been reformatted from the struct to the LD document, it is interoperable with all existing Subjects and Issuers.

Because the traits refer to eachother abstractly, a new Subject / Issuer can be used with all existing Content, and vice versa.

### Credential Witness Flows

Now for a look at the witnessed credential flows. We will look at `Statement`, `Proof` and `Flow` in that order, but first an overview of how witness credential flows work in Rebase:

1) The client asks for `Instructions` from the witness service. The witness service sends human-readable instructions for how the user can work through the flow along with JSON schemas of acceptable request bodies for the next two steps of the flow (statement generation then witnessing).

2) The user follows the instructions and supplies needed information for the statement generation, including a valid Subject. The witness responds with a statement for the user to sign using the Subject supplied earlier in this step.

3) The user signs the statement and performs some challenge that proves the assertation in the challenge (posting the signature to a gist, pasting an auth code from an email, etc). The user sends proof of completion of the challenge to the witness.

4) The witness validates the proof sent by the user, and if it all checks out, issues a credential.
### Statement

`Statement`, is a deceptively simple trait:
```rust
pub trait Statement {
    fn generate_statement(&self) -> Result<String, StatementError>;
}
```

In practice things that implement `Statement` always contain a `Subject` and information to connect to that `Subject`. The implementation of `Statement` for the second example credential looks like:

```rust
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "statement")]
pub struct GitHub {
    pub handle: String,
    pub subject: Subjects,
}

impl Statement for GitHub {
    fn generate_statement(&self) -> Result<String, StatementError> {
        Ok(format!(
            "I am attesting that this GitHub handle {} is linked to the {} {}",
            self.handle,
            self.subject.statement_title()?,
            self.subject.display_id()?
        ))
    }
}
```

When an end user provides a `Subject`, such as an Ethereum wallet and the additional piece of information, a `handle` for a GitHub account, Rebase can kick off a credentialing flow, sending back an attestation of that fact to the end user for them to sign with the `Subject`'s corresponding private key.

Often (at time of writing, all) Statements include a property of the Subjects type. The Subjects type is defined in `src/types/enums/subject.rs`, and at time of writing it looks like:

```rust
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "subject")]
pub enum Subjects {
    #[serde(rename = "pkh")]
    Pkh(Pkh),
    #[serde(rename = "web")]
    Web(Web),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "pkh")]
pub enum Pkh {
    #[serde(rename = "eip155")]
    Eip155(Eip155),
    #[serde(rename = "solana")]
    Solana(Solana),
}

#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "web")]
pub enum Web {
    #[serde(rename = "ed25519")]
    Ed25519(Ed25519),
}
```

This structure encompasses all supported `Subject`s in an enum, then goes on to impl `Subject` for `Subjects`, allowing the `Statement`s to generalize over all current `Subject`s. This approach is very useful for the type restrictions found in the darker corners of WASM compilation. 

The `Subjects` enum also implements a `statement_title` method which is used in the generation of the statement, as seen above. This is separated out so that in the future, internationalization could be implemented only in the `Statement`. The expectation is as a new `Subject` is implemented, it is added to the `Subjects` enum to allow it to be used in all witnessed credentials.

### Proof

`Proof` is also an incredibly simple definition. Defined in terms of a `Content`, `Proof` is simply a structure that when given a statement and signature can be turned into a struct implementing `Content`.

It's defined as:

```rust
pub trait Proof<T>
where
    T: Content,
    Self: Statement,
{
    fn to_content(&self, statement: &str, signature: &str) -> Result<T, ProofError>;
}
```

The example implementation of it looks like: 
```rust
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
#[serde(rename = "proof")]
pub struct GitHub {
    pub gist_id: String,
    pub statement: Stmt,
}

impl Statement for GitHub {
    fn generate_statement(&self) -> Result<String, StatementError> {
        self.statement.generate_statement()
    }
}

impl Proof<Ctnt> for GitHub {
    fn to_content(&self, statement: &str, signature: &str) -> Result<Ctnt, ProofError> {
        Ok(Ctnt {
            gist_id: self.gist_id.clone(),
            handle: self.statement.handle.clone(),
            subject: self.statement.subject.clone(),
            statement: statement.to_owned(),
            signature: signature.to_owned(),
        })
    }
}
```

`Stmt` is a type alias for the `Statement` `impl`ing `GitHub` struct above. Note that `to_content` doesn't do any validation of the proof, just transforms it into a struct that implements `Content`, so it can be used to issue a credential. The validation will be shown in the next step, the `Flow` trait, but first, the `Content` impling struct it turns into is defined as:
```rust
#[derive(Clone, Deserialize, JsonSchema, Serialize)]
pub struct GitHub {
    pub gist_id: String,
    pub handle: String,
    pub subject: Subjects,
    pub statement: String,
    pub signature: String,
}

impl Content for GitHub {
    fn context(&self) -> Result<serde_json::Value, ContentError> {
        // TODO: MAKE THESE URLS POINT ELSEWHERE.
        Ok(json!([
            "https://www.w3.org/2018/credentials/v1",
            {
                "sameAs": "http://schema.org/sameAs",
                "GitHubVerification": "https://example.com/GitHubVerification",
                "GitHubVerificationMessage": {
                    "@id": "https://example.com/GitHubVerificationMessage",
                    "@context": {
                        "@version": 1.1,
                        "@protected": true,
                        "timestamp": {
                            "@id": "https://example.com/timestamp",
                            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
                        },
                        "gistId": "https://example.com/gistId",
                        // "gistVersion":  "https://example.com/gistVersion",
                        "handle": "https://example.com/handle"
                    }
                }
            }
        ]))
    }

    fn evidence(&self) -> Result<Option<OneOrMany<Evidence>>, ContentError> {
        let mut evidence_map = std::collections::HashMap::new();
        evidence_map.insert(
            "handle".to_string(),
            serde_json::Value::String(self.handle.clone()),
        );

        evidence_map.insert(
            "timestamp".to_string(),
            serde_json::Value::String(Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)),
        );

        evidence_map.insert(
            "gistId".to_string(),
            serde_json::Value::String(self.gist_id.clone()),
        );

        let evidence = Evidence {
            id: None,
            type_: vec!["GitHubVerificationMessage".to_string()],
            property_set: Some(evidence_map),
        };

        Ok(Some(OneOrMany::One(evidence)))
    }

    fn subject(&self) -> Result<serde_json::Value, ContentError> {
        Ok(json!({
            "id": self.subject.did()?,
            "sameAs": format!("https://github.com/{}", self.handle)
        }))
    }

    fn types(&self) -> Result<Vec<String>, ContentError> {
        Ok(vec![
            "VerifiableCredential".to_owned(),
            "GitHubVerification".to_owned(),
        ])
    }
}
```
The biggest difference between this definition and the self-issued credential's definition is the use of `evidence`, which records when the gist was looked up and where, but more on that in the next section:

### Flow

A `Flow` is defined in terms of `Content`, `Statement`, `Proof` (which is parameterized by the same `Content`), and a `Statement Response`. The underlying `Flow` implementation includes any information needed to validate the Proof, including things like `api_key`s and max look up limits. A struct implementing `Flow` is expected to be `Deserializable` for use in WASM based libraries.

```rust
#[async_trait(?Send)]
pub trait Flow<C, S, P, R>
where
    C: Content,
    S: Statement,
    P: Proof<C>,
    R: Serialize,
{

    // NOTE: IMPLEMENTED VIA COMPOSITION
    async fn credential<I: Issuer>(&self, proof: &P, issuer: &I) -> Result<Credential, FlowError> {
        /* NOTE: THIS IS AUTOMATICALLY IMPLEMENTED! */
    }

    fn instructions(&self) -> Result<Instructions, FlowError>;

    // NOTE: IMPLEMENTED VIA COMPOSITION
    async fn jwt<I: Issuer>(&self, proof: &P, issuer: &I) -> Result<String, FlowError> {
        /* NOTE: THIS IS AUTOMATICALLY IMPLEMENTED! */
    }

    async fn statement<I: Issuer>(&self, statement: &S, issuer: &I) -> Result<R, FlowError>;

    // NOTE: IMPLEMENTED VIA COMPOSITION
    async fn unsigned_credential<Subj: Subject, I: Issuer>(
        &self,
        proof: &P,
        subj: &Subj,
        issuer: &I,
    ) -> Result<Credential, FlowError> {
        /* NOTE: THIS IS AUTOMATICALLY IMPLEMENTED! */
    }

    async fn validate_proof<I: Issuer>(&self, proof: &P, issuer: &I) -> Result<C, FlowError>;
}
```

An implementor of `Flow` needs to provide three functions, the first, `Instructions` is just a set of human readable instructions on how to get through the flow and some automatically derived JSON Schemas to help the clients send the right format back.

Similarly `statement` often just takes the `statement` arguement, calls `statement.generate_statement()` and returns it. However, in some cases, like e-mail, the witness also generates a challenge at this point. Because we have access to the issuer at this stage, we can use it generate a cryptograpic challenge, then later validate it statelessly.

Finally, `validate_proof` takes a proof and returns the proof's associated content assuming the proof is valid. Proofs can't check their own validity because in the case of things where API access is needed, every proof would have to contain a copy of an API key. Using the `Flow` abstraction to validate the proofs, the `Flow` struct can contain the API key. Additionally, if the `Proof` passed as an argument here contians a challenge generated in the `statement` step, it can be checked here using the same `Issuer`.

The implementor of `Flow` used to generate the second example credential looks like:

```rust
#[async_trait(?Send)]
impl Flow<Ctnt, Stmt, Prf, PostResponse> for GitHubFlow {
    fn instructions(&self) -> Result<Instructions, FlowError> {
        Ok(Instructions { 
            statement: "Enter your GitHub account handle to verify and include in a signed message using your wallet.".to_string(),
            statement_schema: schema_for!(Stmt),
            signature: "Sign the message presented to you containing your email address and additional information.".to_string(),
            witness: "Find the email sent from the witness and copy the code and challenge into the respective form fields.".to_string(),
            witness_schema: schema_for!(Prf) 
        })
    }

    async fn statement<I: Issuer>(
        &self,
        statement: &Stmt,
        _issuer: &I,
    ) -> Result<PostResponse, FlowError> {
        Ok(PostResponse {
            delimitor: self.delimitor.to_owned(),
            statement: statement.generate_statement()?,
        })
    }

    async fn validate_proof<I: Issuer>(&self, proof: &Prf, _issuer: &I) -> Result<Ctnt, FlowError> {
        let client = Client::new();
        let request_url = format!("https://api.github.com/gists/{}", proof.gist_id);
        let re = Regex::new(r"^[a-zA-Z0-9]{32}$")
            .map_err(|_| FlowError::BadLookup("could not generate gist id regex".to_string()))?;

        if !re.is_match(&proof.gist_id) {
            return Err(FlowError::BadLookup("gist id invalid".to_string()));
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            format!("{}", self.user_agent).parse().map_err(|_| {
                FlowError::BadLookup("could not generate header for lookup".to_string())
            })?,
        );

        let res: GitHubResponse = client
            .get(Url::parse(&request_url).map_err(|e| FlowError::BadLookup(e.to_string()))?)
            .headers(headers)
            .send()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?
            .json()
            .await
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        if proof.statement.handle.to_lowercase() != res.owner.login.to_lowercase() {
            return Err(FlowError::BadLookup(format!(
                "handle mismatch, expected: {}, got: {}",
                proof.statement.handle.to_lowercase(),
                res.owner.login.to_lowercase()
            )));
        };
        let s = serde_json::to_string(&res.files)
            .map_err(|e| FlowError::BadLookup(e.to_string()))?;

        for (_k, v) in res.files {
            let object = match v.as_object() {
                None => continue,
                Some(x) => x,
            };

            let str_val = match object.get("content") {
                None => continue,
                Some(x) => x,
            };

            let p = match str_val.as_str() {
                None => continue,
                Some(x) => x,
            };

            let mut a = p.split(&self.delimitor); 
            let txt = a.next(); 
            let txt_sig = a.next();

            match (txt, txt_sig) {
                (Some(stmt), Some(sig)) => {
                    proof.statement.subject.valid_signature(stmt, sig).await?;
                    return Ok(proof.to_content(stmt, sig)?)
                }
                _ => continue
            }
            
        }

        Err(FlowError::BadLookup(
            format!("Failed to find files in: {}", s),
        ))
    }
}
```

This implementation of `validate_proof` looks up the gist, validates the username matches the name in the statement, validates the signature is the statement signed by the subject. It uses it's own internal `delimitor` property to split the gists to find statment and signature. Similiarly internal properties like `api_key` or `email_address` can be used to handle all sort of flows. 

Some flows don't even require a post, like the email flow, which statelessly creates and validates challenges to prove email address ownership.

### Conclusion

We've now covered each of the types defined and utilized by Rebase. Hopefully you feel comfortable enough to add new Subjects, Issuers, Contents, and Flows. If you're looking for more information on how to run a witness service or how to create a client for an exisiting witness service, check out these two libraries ([witness SDK](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk)), ([JS Client](https://github.com/spruceid/rebase/tree/main/js/rebase-client)). If you want to see examples in action, here's the [credential faucet](https://rebase.pages.dev/), that site's [code base](https://github.com/spruceid/rebase/tree/main/demo/dapp), or the [code base](https://github.com/spruceid/rebase/tree/main/demo/witness) of the Cloudflare worker witness service that backs that site.