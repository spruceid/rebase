# Rebase Witness SDK

This SDK enables creating server-side "Witness" ([Verifiable Credential](https://www.w3.org/TR/vc-data-model/) issuing) services over HTTP as well as creating corresponding clients. Because it is developed in Rust, it can by called by many languages through FFIs and can compile to WebAssembly (WASM) for use in the browser (available [in this package](../../js/rebase-client)).

* A full working example of the witness services can be found [here](../rebase_cf_worker/) implemented as [Cloudflare Worker](https://workers.cloudflare.com/). 
* A full working example of a UI that utilizes the client has a repo [here](../../demo/dapp). 
* An online implementation [here](https://rebase.pages.dev), where it points to a live CF worker witness.

This repository combines all of the `flow`s defined in the [Core Library](../rebase/) into a single `WitnessFlow` struct found [in this file](./src//types.rs) with all flows being optional. A witness service can then configure as many or as few of the `flow`s as they so choose. A good example of usage of this configuration is found in the CF worker project linked above.

Each time a new flow is added, the `WitnessFlow` and it's `Flow<...>` definition must be updated in order to make the new `flow` available to consumers of the library. 

To make use of witnesses developed by this library in websites, see [Rebase Client](../../js/rebase-client/) for the Typescript+WASM implementation of the [client defined here](./src/client.rs).