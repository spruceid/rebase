# rebase

Rebase is a set of libraries for handling the witnessing of cryptographically verifiable claims, and the issuance of Verifiable Credentials (VC) based on this programmatic witnessing, as well as self-issued credentials. Rebase simplifies the process of creating links between identity providers, or self-attested claims using VCs by providing a convenient wrapper around [`ssi`](https://github.com/spruceid/ssi). Rebase is intended for a wide variety of uses ranging from server-side "witness" services, to VC reading validation services, to in-browser usage via WASM. 

## Architectural Overview

Several projects are hosted in this repo.
* The [main library](https://github.com/spruceid/rebase/tree/main/rust/rebase) written in Rust supports the creation of VCs from simpler structs, the signing of VCs using an `Issuer` abstraction, and witness flows for public issuers of VCs (along with their consuming clients).
* The [witness library](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk) also written in Rust which supports specifically applications seeking to act as Issuer Witnesses or clients of Issuer Witnesses. Creates an opinionated implementation of the main library allowing users to build on top of it quickly.
* The [client library](https://github.com/spruceid/rebase/tree/main/js/rebase-client) published to NPM using Rust->WASM compilation, allowing in browser usage of the witness library for usage as a client or as a self-issuer in browser.
* [Demos!](https://github.com/spruceid/rebase/tree/main/demo) Which make use of all of the above.

## Examples

There's an existing [credential faucet](https://rebase.pages.dev/) which is the built version of the `demo/dapp` site, and makes use of a deployed Witness service which in turn is the built version of the `demo/witness` codebase.

The `demo` directory includes a Cloudflare Worker that acts as a server-side witness (`demo/witness`) and a front-end UI for interacting with the witness (`demo/dapp`). Installation and usage instructions are found in those respective directories, but the high-level overview is given here. 

The Cloudflare Worker acts a proof-of-concept that Rebase can be packaged for WASM environments, including the browser. Otherwise, it essentially functions as a tiny HTTP server. It contains 3 routes, `/instructions` where the client can retrieve user-facing instructions for a witness flow along with JSON schema representations of the expected user input, `/statement`, where the client is expected to post a struct that implements `Statement` and then receives the generated statement from the witness and `/witness` where a struct that implements `Proof` is posted, and the witness uses its generator to produce a VC (assuming all the details check out).

The UI is a thin client that simply gathers the information required to generate the statement, interacts with browser extensions to get the user to sign the statement, informs the user where to post the statement (if necessary), then gathers the information on the location of the post (again, if necessary), returns it to the witness for a VC, then displays the VC and allows the user to download it.

## Current Features

Current Schemas Defined:
* basic_post (self-issued)
* basic_profile (self-issued)
* dns
* email
* github
* reddit
* same (links two keys)
* soundcloud
* twitter

Current Witness flows:
* dns
* email
* github
* reddit
* same (links two keys)
* soundcloud
* twitter

Current Subjects:
* ethereum
* ed25519
* solana

Current Issuers:
* ed25519 (via did:web)
