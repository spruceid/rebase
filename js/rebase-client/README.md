# Rebase Client

This Rust-to-WASM compiled library is a thin wrapper around the client portion of the [Rebase Witness SDK](). It makes several design choices that make usage in WASM easier. The `Client` struct exposed by the library (and made available to JavaScript consumers) has an implementation that looks like:

```rust
#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(statement: String, jwt: Option<String>, ld: Option<String>) -> Result<Client, String> {
        // ...
    }
}
```

Unlike the underlying Rust library which this library imports, the `Client` doesn't expose methods, but is an argument passed into the exposed functions:
```rust
#[wasm_bindgen]
pub async fn statement(client: Client, req: String) -> Promise {
    // ...
}

#[wasm_bindgen]
pub async fn jwt(client: Client, req: String) -> Promise {
    // ...
}

#[wasm_bindgen]
pub async fn ld(client: Client, req: String) -> Promise {
    // ...
}
```

This because of limitations of `future_to_promise` library which would cause problems if the `client` was moved, but it must be to allow `async` functions to be called.

This causes the `client` to be dropped after each usage. Easy usage of the client in JavaScript is described [here]().