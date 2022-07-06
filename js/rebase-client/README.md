# Rebase Client

This Rust-to-WASM compiled library is a thin wrapper around the client portion of the [Rebase Witness SDK](). It makes several design choices that make usage in WASM easier. 

## Rust implementation
The `Client` struct exposed by the library (and made available to JavaScript consumers) has an implementation that looks like:

```rust
#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(statement: String, jwt: Option<String>, ld: Option<String>) -> Result<Client, String> {
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

Once the client is instanciated, the user simply has to pass in `req`s that conform to JSON stringified requests described in detail [here](), where the resulting `Promise` contains a JSON stringified responses described in the linked doc.

## JavaScript Usage
Concrete usage of this library in JS is found [here](). The client's constructor accepts up to three urls assumed to be pointed at a witness service created using the library found [here](). The first URL expected is for statement generation, the second is for JWT credential generation, and the third for LD credential generation.

The statement URL is required and at least one of the two optional URLs must be provided. All of the following would be valid:

```JavaScript
import { Client } from "@rebase-xyz/rebase-client";

const statementUrl = "https://example.com/statement";
const jwtUrl = "https://example.com/witness/jwt";
const ldUrl = "https://example.com/witness/ld";

let client = new Client(statementUrl, jwtUrl);
client = new Client(statementUrl, jwtUrl, null);
client = new Client(statementUrl, null, ldUrl);
client = new Client(statementUrl, jwtUrl, ldUrl);
```

All of the following would be invalid.
```JavaScript
client = new Client(null, null, null);
client = new Client(null, jwtUrl, null);
client = new Client(null, null, ldUrl);
client = new Client(null, jwtUrl, ldUrl);
client = new Client(statementUrl, null, null);
```

Once a valid client has been constructed, it can be used like so (where `req` is a JSON stringified valid request):
```JavaScript
let res = await client.statement(req);
```
This would produce a JSON stringified version of the StatementRes found [here]().
```JavaScript
let jwtRes = await client.jwt(req);
let ldRes = await client.ld(req);
```
A corresponding `jwtUrl` or `ldUrl` must be provided at config time to use `jwt` and `ld` methods. These would both produce a JSON stringified version of the WitnessRes found [here]()
