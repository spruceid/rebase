# Rebase Client

This Rust-to-WASM compiled library is a thin wrapper around the client portion of the [Rebase Witness SDK](../../rust/rebase_witness_sdk). It makes several design choices that make usage in WASM easier. An example of this client in use is [here](../../demo/dapp/).

Documentation for the Rebase Client is coming soon. For now, please see the [demo front-end project for a working example](../../demo/dapp/).

However, this client includes Typescript types generated via the underlying Rust code, so all arguments are well specified and de/serialization code at the borders between JS and WASM have largely been avoided (the only place it's still needed is creating the client initially, shown below). This should at least make it easier to reason about.

## Installation

Install through `npm`:
```
$ npm i @spruceid/rebase-client
```

## Usage

Basic usage is as follows:
```ts
// These are the types that will be regularly used by the application
import { Client, defaultClientConfig, Types } from "@spruceid/rebase-client";
// This is the inner, untyped WASM client that the above types will wrap.
import { WasmClient } from "@spruceid/rebase-client/wasm";

// Create the client, this is done to simplify build steps and wrap the untyped client
// in it's auto-generated types.
const client = new Client(new WasmClient(JSON.stringify(defaultClientConfig())));

// Get input from the user, format it into the request type for the client.
const getUserStatement = (): Types.BasicProfileAttestationStatement => {
    // ...
};

// Save user input
const s = getUserStatement();

// Get the statement from the witness
let statementRes = await client.statement(s);

// Have a way to sign bytes using the subject provided in getUserStatement
const getUserSignature = (statement: string): string => {
    // ...
};

// Issue the credential, in this case, a JWT.
let witnessRes = await client.witness_jwt(Types.BasicProfileAttestationProof {
    statement: s,
    signature: getUserSignature(statementRes.statement)
});

// Do something with the issued credential!
console.log(witnessRes.jwt)

```
