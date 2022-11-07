# Rebase Witness SDK

This software development kit enables creating server-side "Witness" ([Verifiable Credential](https://www.w3.org/TR/vc-data-model/) issuing) services over HTTP as well as creating corresponding clients. Because it is developed in Rust, this allows for many languages to call it through FFIs and for compilation of the client supporting code into WebAssembly (available [in this package](https://github.com/spruceid/rebase/tree/main/js/rebase-client), published to [npm](https://www.npmjs.com/package/@rebase-xyz/rebase-client)).

## Creating Witness Services

A full working example of the witness services can be found [here](https://github.com/spruceid/rebase/tree/main/demo/witness) implemented as [Cloudflare Worker](https://workers.cloudflare.com/). The `types` file of the library contains the majority of the implementation details including the structures used by the `client` portion. The bulk of the code in the Witness SDK wraps up the various concrete implementations of the types specified in the [main rebase library](https://github.com/spruceid/rebase/tree/main/rust/rebase) into enums. 

All `Content` structs (i.e. files matching `rebase/rust/rebase/src/content/*.rs`) are wrapped into `Contents`: 
```rust
#[derive(Deserialize, Serialize)]
pub enum Contents {
    Dns(DnsCtnt),
    Email(EmailCtnt),
    GitHub(GitHubCtnt),
    Reddit(RedditCtnt),
    SoundCloud(SoundCloudCtnt),
    Twitter(TwitterCtnt),
    TwoKey(TwoKeyCtnt),
}
```

Then the trait `Content` is `impl`'d for Contents. The same thing occurs for `Statement`(`s`) and `Proof`(`s`). The `Flow` structs are unified under a `WitnessFlow` struct that looks like:
```rust
#[derive(Deserialize, Serialize)]
pub struct WitnessFlow {
    dns: DnsFlow,
    email: Option<EmailFlow>,
    github: Option<GitHubFlow>,
    reddit: RedditFlow,
    soundcloud: Option<SoundCloudFlow>,
    twitter: Option<TwitterFlow>,
    two_key: TwoKeyFlow,
}
```
The flows that are not optional have no internal properties, thus are always available. Once all of these have been defined, it becomes possible to implement:
```rust
impl Flow<Contents, Statements, Proofs, StatementRes> for WitnessFlow {
    // ...
}
```

The result is that `WitnessFlow` automatically can handle all of the contained flows generically, based on user input. This greatly simplifies the amount of code required to stand up a general witness service, as seen in the [demo witness codebase](https://github.com/spruceid/rebase/tree/main/demo/witness).

## Creating Clients
The client is the corresponding consumer of the witness' service. The `Client` implementation provided by this library looks like:

```rust
impl Client {
    pub fn new(endpoints: Endpoints) -> Result<Client, ClientError> {
        // ...
    }

    pub async fn instructions(&self, req: InstructionReq) -> Result<serde_json::Value, ClientError> {
        // ...
    }

    pub async fn statement(&self, req: StatementReq) -> Result<StatementRes, ClientError> {
        // ...
    }

    pub async fn jwt(&self, req: WitnessReq) -> Result<WitnessJWTRes, ClientError> {
        // ... 
    }

    pub async fn ld(&self, req: WitnessReq) -> Result<WitnessLDRes, ClientError> {
        // ...
    }
}
```

Once a client is created, it is able to exchange `StatementReq`s for `StatementRes`s, `InstructionReq`s for the structure described [here](https://github.com/spruceid/rebase/blob/main/rust/rebase/src/types/types.rs#L111), and exchange `WitnessReq`s for `WitnessJWTRes` or `WitnessLDRes` depending on what is requested. This is done through interaction with a witness specified at time of `Client` creation. A client is created by providing an `Endpoints` struct which looks like:

```rust
pub struct Endpoints {
    pub jwt: Option<Url>,
    pub ld: Option<Url>,
    pub statement: Url,
    pub instructions: Url,
}
```

Each of the properties represents a URL of a witness from the previous section which allows for the exchange to occur. At least one of the optional properties must be provided, or `Client::new` will return an error. Once this is provided, the client is perfectly re-usable.

An example of usage of a WASM client is found in the [demo dapp codebase](https://github.com/spruceid/rebase/tree/main/demo/dapp), which can be seen deployed [here, at the Rebase credential faucet](https://rebase.pages.dev/).
