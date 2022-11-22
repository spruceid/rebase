# Rebase Client

This Rust-to-WASM compiled library is a thin wrapper around the client portion of the [Rebase Witness SDK](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk). It makes several design choices that make usage in WASM easier. 

## Install
```
$ npm i @rebase-xyz/rebase-client
```

## Common Usage:
```javascript
import { Client } from "@rebase-xyz/rebase-client";

// const witnessUrl = ...

const statementUrl = `${witnessUrl}/statement`;
const instructionsUrl = `${witnessUrl}/instructions`;
const jwtUrl = `${witnessUrl}/witness_jwt`;
const ldUrl = `${witnessUrl}/witness_ld`;

export const client = new Client(instructionsUrl, statementUrl, jwtUrl, ldUrl);

const instructionsReq = {
    type: "github" // or "email" or "twitter" or "reddit" or more!
};

let instructionsRes = await client.instructions(instructionsReq)
instructionRes = JSON.parse(instructionsRes);

// const statementReq = ...

let statementRes = await client.statement(statementReq);
statementRes = JSON.parse(statementRes);

// const proofReq = makeProofFromStatement(statmentRes);

// Get a JWT credential.
let proofRes = await client.jwt(proofReq);
proofRes = JSON.parse(proofRes);
let credential;
if (proofRes.jwt) {
    credential = proofRes.jwt;
}

// Get a LD credential
let proofRes = await client.ld(proofReq);
proofRes = JSON.parse(proofRes);
if (proofRes.credential) {
    credential = proofRes.credential;
}
```

## Rust implementation
The `Client` struct exposed by the library (and made available to JavaScript consumers) has an implementation that looks like:

```rust
#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(
        instructions: String,
        statement: String,
        jwt: Option<String>,
        ld: Option<String>,
    ) -> Result<Client, String> {
        // ...
    }

    pub fn instructions(&self, req: String) -> Promise {
        // ...
    }

    pub  fn statement(&self, req: String) -> Promise {
        // ...
    }

    pub fn jwt(&self, req: String) -> Promise {
        // ...
    }

    pub fn ld(&self, req: String) -> Promise {
        // ...
    }
}
```

Once the client is instanciated, the user simply has to pass in `req`s that conform to JSON stringified requests described in detail [here](https://github.com/spruceid/rebase/blob/main/demo/witness/endpoints.md), where the resulting `Promise` contains a JSON stringified response described in the linked doc.

## JavaScript Usage
Concrete usage of this library in JS is found [here](https://github.com/spruceid/rebase/blob/main/demo/dapp/src/util/witness.ts). The client's constructor accepts up to four URLs assumed to be pointed at a witness service created using the library found [here](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk). 

The first URL expected is for instructions retreival, the second is for statement generation, the third is for JWT credential generation, and the fourth for LD credential generation.

The instructions and statement URLs are required and at least one of the two optional URLs must be provided. All of the following would be valid:

```JavaScript
import { Client } from "@rebase-xyz/rebase-client";

const statementUrl = "https://example.com/statement";
const instructionsUrl = "https://example.com/instructions";
const jwtUrl = "https://example.com/witness/jwt";
const ldUrl = "https://example.com/witness/ld";

let client = new Client(instructionsUrl, statementUrl, jwtUrl);
client = new Client(instructionsUrl, statementUrl, jwtUrl, null);
client = new Client(instructionsUrl, statementUrl, null, ldUrl);
client = new Client(instructionsUrl, statementUrl, jwtUrl, ldUrl);
```

All of the following (and more) would be invalid.
```JavaScript
client = new Client();
client = new Client(null, null);
client = new Client(null, null, null, null);
client = new Client(null, statementUrl, jwtUrl, null);
client = new Client(instructionsUrl, null, jwtUrl, null);
client = new Client(null, null, jwtUrl);
client = new Client(null, statementUrl, jwtUrl, ldUrl);
client = new Client(instructionsUrl, null, jwtUrl, ldUrl);
client = new Client(instructionsUrl, statementUrl, null, null);
```

Once a valid client has been constructed, it can be used like so (where `req` is a JSON stringified valid request):
```JavaScript
let res = await client.statement(req);
```

This would produce a JSON stringified version of the `StatementRes`. Instructions requests work the same way using `client.instructions`. The exact structure of the responses are detailed in the [endpoints document](https://github.com/spruceid/rebase/blob/main/demo/witness/endpoints.md).

```JavaScriptThis Rust-to-WASM compiled library
let jwtRes = await client.jwt(req);
let ldRes = await client.ld(req);
```

A corresponding `jwtUrl` or `ldUrl` must be provided at config time to use `jwt` and `ld` methods. These would both produce a JSON stringified version of the WitnessJWTRes/WitnessLDRes described in the [endpoints document](https://github.com/spruceid/rebase/blob/main/demo/witness/endpoints.md).

Once the credential is available to the frontend, developers can then store the credential externally or present it to another service that is seeking the information attested to by the credential.