# Rebase

Rebase offers an extensible, opinionated, trait-based approach to building credentialing systems on top of the foundation of [ssi](https://github.com/spruceid/ssi).

The most comprehensive documentation of the foundational Rebase library and it's associated SDKs are found [at this site](https://www.spruceid.dev/rebase/rebase). A higher-level, more abstract overview is given here.

Rebase is a library that enables users and projects to easily issue their own Verifiable Credentials (VCs). It supports two types of credential flows: self-signed credentials and witnessed credentials, examples of each of these can be found [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/credentials-and-how-they-are-made). 

Rebase provides points of extension to add new credentialing flows of either type and new cryptographic subjects/issuers. All subjects, issuers, and credentialing flows can be mixed and matched. 

The goal of Rebase is to provide these tools for both client and server side usage. For this reason, WASM is a first class use-case. 

The output of Rebase libraries are [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/), both [Linked Data Proof](https://www.w3.org/standards/semanticweb/data) and [JWT](https://www.rfc-editor.org/rfc/rfc7519.html) formats are supported. Rebase uses [Decentralized Identifiers (DIDs)](https://www.w3.org/TR/did-core/) to represent the subjects and issuers of these credentials. 

## Usage

If you want a client to interact with an existing witness, or if you want to create your own witness service, take a look at the [Witness SDK](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk), built on top of this library. 
## Current Features

Current credentials supported:
* basic_post (self-issued)
* basic_profile (self-issued)
* dns
* email
* github
* reddit
* same (links two Subject instances)
* soundcloud
* twitter

Current Witness flows:
* dns
* email
* github
* reddit
* same (links two Subject instances)
* soundcloud
* twitter

Current Subjects:
* ethereum
* ed25519
* solana

Current Issuers:
* ed25519 (via did:web)

## Rebase Basics

### Motivation

The goal of the Rebase library is to provide tools that enable the calling application to easily create VCs, no matter whether it's a client self-issuing a credential or it's a server-side witness service transforming an assertation into a credential. 

A detailed look into the architecture that supports the two types of Rebase credentialing flows is found [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase).

A simple overview of what the content of the credentials represent is described [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/the-anatomy-of-a-credential).
### Overview of Architecture

Rebase's `src` folder is further divided into 8 sub-folders which interact to allow calling applications to issue credentials. These sub-folders are:
* `content`
* `flow`
* `issuer`
* `proof`
* `statement`
* `subject`
* `test_util`
* `types`

As one might expect, `test_util` only contains code relevant for the automated test cases. Similarly, `types` contains a list of type definitions without concrete implementations, as well as all error variants used by the rest of the codebase. 

The remaining modules form a dependency chain that looks like:

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

Here are detailed descriptions of each module, going from most primative to most composed:

* [Subject](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/subject)
* [Issuer](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/issuer)
* [Content](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/content)
* [Statement](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/statement)
* [Proof](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/proof)
* [Flow](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/flow)

 Information on how to implement a self-issued credential flow is found [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/implementing-self-issued-credentials), and for a witness flow, check [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/overview-of-witness-flows).

An examination of each supported witness flow is found [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/detailed-descriptions-of-each-witness-flow).

Links to specific witness flow descriptions are found:

* [DNS](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/dns)
* [Email](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/email)
* [GitHub](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/github)
* [Reddit](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/reddit)
* [Same](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/same)
* [SoundCloud](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/soundcloud)
* [Twitter](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow/twitter)
## Other Resources

[This is primary source of Rebase documentation](https://www.spruceid.dev/rebase/rebase), and should be able to answer questions unaddressed here.

If you want to see examples in action, here's the [credential faucet](https://rebase.pages.dev/), that site's [codebase](https://github.com/spruceid/rebase/tree/main/demo/dapp), and the [codebase](https://github.com/spruceid/rebase/tree/main/demo/witness) of the Cloudflare worker witness service that backs that site.

If you're looking for more information on how to run a witness service or how to create a client for an existing witness service, check out these two libraries ([witness SDK](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk)), ([JS Client](https://github.com/spruceid/rebase/tree/main/js/rebase-client)). 
 