# Rebase

## Projects

Several projects are hosted in the Rebase GitHub repo.
* The [main library](./rust/rebase) written in Rust supports the creation of VCs from simpler structs, the signing of VCs using an `Issuer` abstraction, and witness flows for public issuers of VCs (along with their consuming clients).
* The [witness library](./rust/rebase_witness_sdk) also written in Rust which supports specifically applications seeking to act as Issuer Witnesses or clients of Issuer Witnesses. Creates an opinionated implementation of the main library allowing users to quickly build on top of it.
* The [client library](./js/rebase-client) published to NPM using Rust->WASM compilation, allowing in browser usage of the witness library in browser.
* The [CloudFlare worker](./rust/rebase_cf_worker) which is an open-source codebase of the SpruceID Rebase Witness and serves as a working example / demo of the witness library.
* The [demo dapp](./demo/dapp) which is a small UI utilizing the client library to interact with the CloudFlare worker.

## Examples

There's an existing [credential faucet](https://rebase.pages.dev/) which is the built version of the `demo/dapp` site, and makes use of a deployed witness service which in turn is the built version of the `rebase_cf_worker` codebase.
