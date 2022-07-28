# Rebase Witness SDK

This software development kit enables creating server-side "Witness" ([Verifiable Credential]() issuing) services over HTTP as well as creating corresponding clients. Because it is developed in Rust, this allows for many languages to call it through FFIs and for compilation of the client supporting code into WebAssembly (available [in this package]()).

## Creating Witness Services

A full working example of the witness services can be found [here]() implemented as [Cloudflare Worker](). The `witness` portion of the library contains the majority of the implementation details including the structures used by the `client` portion. The structures used are:

```rust
#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    pub opts: StatementTypes,
}

#[derive(Deserialize, Serialize)]
pub struct StatementRes {
    pub statement: String,
    pub delimitor: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: ProofTypes,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessJWTRes {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessLDRes {
    pub credential: Credential,
}
```

[StatementTypes]() and [ProofTypes]() are enums representing all of the possible witness flows supported by the underlying [Rebase]() library. The advantage of this approach is that addition variants are added to underlying library, they are automatically supported by this SDK.

The response types (`StatementRes`, `WitnessJWTRes`, and `WitnessLDRes`) are universal for all witness flows, making developing a consumer of the witness flows automatically backwards compatible to new flows. The [Credential]() type found in the body of the `WitnessLDRes` comes from the [SSI]() library.

The exposed functions have the following signatures:

```rust
pub async fn statement(req: StatementReq) -> Result<StatementRes, WitnessError> {
    // ...
}

pub async fn witness_jwt<T: SignerType, S: Signer<T>>(
    witness_request: WitnessReq,
    generator: &WitnessGenerator,
    signer: &S,
) -> Result<WitnessJWTRes, WitnessError> {
    // ...
}

pub async fn witness_ld<T: SignerType, S: Signer<T>>(
    witness_request: WitnessReq,
    generator: &WitnessGenerator,
    signer: &S,
) -> Result<WitnessLDRes, WitnessError> {
    // ...
}
```

`Signer`, `SignerType`, and `WitnessGenerator` correspond to the underlying Rebase library, and are better described in it's [README](). `WitnessGenerator` is just a re-exported type of Rebase's `Generator` type. A concrete implementation can be found in the `demo/witness` section of the [Rebase repository]().

The most common way to structure a witness service is to have a constant `Signer` and `WitnessGenerator` which are composed with incoming `StatementReq`/`WitnessReq` from clients, then returning the results.

## Creating Clients
The client is the corresponding consumer of the witness' service. The `Client` implementation provided by this library looks like:

```rust
impl Client {
    pub fn new(endpoints: Endpoints) -> Result<Client, ClientError> {
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

Once a client is created, it is able to exchange `StatementReq`s for `StatementRes`s and exchange `WitnessReq`s for `WitnessJWTRes` or `WitnessLDRes` depending on what is requested. This is done through interaction with a witness specified at time of `Client` creation. A client is created by providing an `Endpoints` struct which looks like:

```rust
pub struct Endpoints {
    pub jwt: Option<Url>,
    pub ld: Option<Url>,
    pub statement: Url,
}
```

Each of the properties represents a URL of a witness from the previous section which allows for the exchange to occur. At least one of the optional properties must be provided, or `Client::new` will return an error. Once this is provided, the client is perfectly re-usable, though in the examples where WASM is involved, the client is created on a per-request basis. This pattern is elaborated on in their READMEs.