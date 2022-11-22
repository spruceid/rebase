# Rebase Witness SDK

This SDK enables creating server-side "Witness" ([Verifiable Credential](https://www.w3.org/TR/vc-data-model/) issuing) services over HTTP as well as creating corresponding clients. Because it is developed in Rust, it can by called by many languages through FFIs and can compile to WebAssembly (WASM) for use in the browser (available [in this package](https://github.com/spruceid/rebase/tree/main/js/rebase-client), published to [npm](https://www.npmjs.com/package/@rebase-xyz/rebase-client)).

* A full working example of the witness services can be found [here](https://github.com/spruceid/rebase/tree/main/demo/witness) implemented as [Cloudflare Worker](https://workers.cloudflare.com/). 
* A full working example of a UI that utilizes the client has a repo [here](https://github.com/spruceid/rebase/tree/main/demo/dapp). 
* An online implementation [here](https://rebase.pages.dev), where it points to a live worker.

To understand the underlying repo structure and the Rebase ecosystem in general, [this page](https://www.spruceid.dev/rebase/core-library) covers the foundational Rebase library, [this page](https://www.spruceid.dev/rebase/witness-sdk) covers this specific SDK, and [this page](https://www.spruceid.dev/rebase) contains documentation on all related Rebase libs.

## Creating Witness Services

### The Witness Codebase

The primary use-case of this library is to provide types to make the creation of public witness and credential issuance services as easy as possible. The creation of clients is supported but seperate concern outlined [here](#creating-client-applications). The majority of this library is found in the `src/types.rs` file which defines everything we need to create a Witness service. The primary struct used to create Witness services is defined as so:

```rust
#[derive(Deserialize, Serialize)]
pub struct WitnessFlow {
    dns: Option<DnsFlow>,
    email: Option<EmailFlow>,
    github: Option<GitHubFlow>,
    reddit: Option<RedditFlow>,
    same: Option<SameFlow>,
    soundcloud: Option<SoundCloudFlow>,
    twitter: Option<TwitterFlow>,
}
```

Each flow is individually supportable, simply omitting a configuration will cause the Witness to return errors for attempts to use those flows. Each flow is detailed in [the main library's documentation](https://www.spruceid.dev/rebase/core-library/detailed-descriptions-of-each-witness-flow), and the `Flow` trait in specific is discussed [here](https://www.spruceid.dev/rebase/core-library/a-tour-of-rebase/flow).

With that base knowledge, the easiest way to think about the `WitnessFlow` struct is that it's an abstraction around all `Flow`s so that the Witness can statelessly handle all requests for each of the steps (`instructions` -> `statement` -> `proof`), without flow specific logic being required.

The top-level implementation of the `Flow` trait for `WitnessFlow` looks like:
```rust
#[async_trait(?Send)]
impl Flow<Contents, Statements, Proofs> for WitnessFlow {
    // ...
}
```

The `Contents`, `Statements`, and `Proofs` enums are each wrappers around all structs implementing the associated trait (`Content`, `Statement`, and `Proof`) exposed in the underlying Rebase lib. As an example, `Statements` looks like:
```rust
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename = "opts")]
pub enum Statements {
    #[serde(rename = "dns")]
    Dns(DnsStmt),
    #[serde(rename = "email")]
    Email(EmailStmt),
    #[serde(rename = "github")]
    GitHub(GitHubStmt),
    #[serde(rename = "reddit")]
    Reddit(RedditStmt),
    #[serde(rename = "same")]
    Same(SameStmt),
    #[serde(rename = "soundcloud")]
    SoundCloud(SoundCloudStmt),
    #[serde(rename = "twitter")]
    Twitter(TwitterStmt),
}
```
with `rebase::statement::dns::Dns` aliased to `DnsStmt` and so forth.

The result of this is that given an instance of `Statements`, one can simply pass that instance to `WitnessFlow.statement` and get the expected result (presuming the flow is implemented in WitnessFlow).

This pattern is matched for `Contents` and `Proofs`, as well as instructions (via `InstructionsType`).

To support a new flow, a struct `impl`ing the flow has to be added (wrapped in an `Option`) to the `WitnessFlow` struct, then an entry added to each of the enums mentioned above (`InstructionsType`, `Statements`, `Contents`, and `Proofs`).

Then adding support in the `WitnessFlow`'s `impl` of `Flow` for each step. A very complex macro could probably decrease this work, but it isn't particularly sizable as is.

Because the foundational library abstracts over `Subjects` using a similar strategy, updating this library to take advantage of new `Subjects` and `Issuers` is as simple as updating the Rebase dep, then recompiling.

The shape of the requests and responses that the `WitnessFlow` can handle are also defined in `src/types.rs` like so:

```rust
#[derive(Deserialize, Serialize)]
pub struct InstructionsReq {
    #[serde(rename = "type")]
    pub instruction_type: InstructionsType,
}

#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    pub opts: Statements,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: Proofs,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WitnessJWTRes {
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessLDRes {
    pub credential: Credential,
}
```

If a user supplies a valid `InstructionsReq`, then a valid `StatementReq`, and a valid `WitnessReq`, the user will receive a credential, either in the JWT format or LD proof format, depending on how the client requested. This is generalized to make the development of clients much easier.

### Example implementation:

An existing implementation of a Rebase witness service is found [here](https://github.com/spruceid/rebase/tree/main/demo/witness). This is a WASM-based CloudFlare worker build, which uses a thin JS wrapper around the actual WASM core to handle the actual witnessing.

The Rust (compiled to WASM) portion of the witness code looks like:
```rust
#[derive(serde::Deserialize)]
pub struct Opts {
    witness: WitnessFlow,
    did: String
}

#[wasm_bindgen]
pub async fn instructions(req: String, opts: String) -> Promise {
    future_to_promise(async move {
        let req: InstructionsReq = jserr!(serde_json::from_str(&req));
        let opts: Opts = jserr!(serde_json::from_str(&opts));
        let res = jserr!(opts.witness.handle_instructions(&req).await);
        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}

#[wasm_bindgen]
pub async fn statement(secret: String, req: String, opts: String) -> Promise {
    future_to_promise(async move {
        let opts: Opts = jserr!(serde_json::from_str(&opts));
        let issuer = jserr!(DidWebJwk::new(&opts.did, &secret, "controller"));
        let req: StatementReq = jserr!(serde_json::from_str(&req));
        let res = jserr!(opts.witness.handle_statement(&req, &issuer).await);
        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}

#[wasm_bindgen]
pub async fn witness(secret: String, witness_request: String, opts: String) -> Promise {
    future_to_promise(async move {
        let opts: Opts = jserr!(serde_json::from_str(&opts));
        let issuer = jserr!(DidWebJwk::new(&opts.did, &secret, "controller"));
        let witness_request: WitnessReq = jserr!(serde_json::from_str(&witness_request));
        let res = jserr!(opts.witness.handle_jwt(&witness_request, &issuer).await);

        Ok(jserr!(serde_json::to_string(&res)).into())
    })
}
```

Ignoring `jserr!` which is used to manipulate Rust errors into JSValues, the code is extremely straight forward. The `Opts` struct is comprised of the DID of the issuer (in this case a Ed25519 DID Web key) and the WitnessFlow structure. It also assumes that a `secret` parameter will be a stringified [JWK](https://www.rfc-editor.org/rfc/rfc7517) corresponding to the DID, and it will be passed for `statement` and `witness` requests.

Once the `Opts` and JWK structures are deseralized, the DID is combined with the `secret` to make a valid issuer, then is passed to `opts.witness` depending on the request type. This witness only supports JWT issuance, but it would be trivial to create a witness that handles both.

On the JavaScript side, the following function is used to generate the Witness Opts:

```javascript
function witnessOpts() {  
  let o = {};
  o.dns = {};
  o.reddit = {};
  o.same = {};

  if (GITHUB_USER_AGENT) {
    o.github = {
      user_agent: GITHUB_USER_AGENT,
      delimitor: "\n\n"
    }
  }

  if (SOUNDCLOUD_CLIENT_ID) {
    let limit = parseInt(SOUNDCLOUD_LIMIT);
    let offset = parseInt(SOUNDCLOUD_MAX_OFFSET);
    if (!isNaN(limit) && !isNaN(offset)) {
      o.soundcloud = {
        client_id: SOUNDCLOUD_CLIENT_ID,
        limit,
        max_offset: offset,
      }
    }
  }

  if (TWITTER_BEARER_TOKEN) {
    o.twitter = {
      api_key: TWITTER_BEARER_TOKEN,
      delimitor: "\n\n"
    }
  }

  let useSendGrid = SENDGRID_BEARER_TOKEN 
    && SENDGRID_FROM_ADDRESS 
    && SENDGRID_FROM_NAME
    && SENDGRID_SUBJECT_NAME
    && SENDGRID_MAX_ELAPSED_MINS
    && !isNaN(parseInt(SENDGRID_MAX_ELAPSED_MINS));

  if (useSendGrid) {
    o.email = {
      api_key: SENDGRID_BEARER_TOKEN,
      from_addr: SENDGRID_FROM_ADDRESS,
      from_name: SENDGRID_FROM_NAME,
      max_elapsed_minutes: parseInt(SENDGRID_MAX_ELAPSED_MINS),
      subject_name: SENDGRID_SUBJECT_NAME,
    }
  }

  return o
};

const opts = {
  witness: witnessOpts(),
  did: DID_WEB
}
```

Using [CloudFlare's environmental secrets for workers](https://blog.cloudflare.com/workers-secrets-environment/), the JavaScript puts together a serializable opts object for use in the WASM. This `opts` const can be used for every incoming request. Its use ends up looking like:

```javascript
async function wtns(request) {
  try {
    await instance;
    const h = request.headers;

    const contentType = h.get('content-type') || '';

    if (contentType.includes('application/json')) {
      let body = await request.json();

      const credential = await witness(REBASE_SK, JSON.stringify(body), JSON.stringify(opts));

      return new Response(credential, {status: 200, headers: headers});

    } else {
      throw new Error(`Expected content-type application/json, got: ${contentType}`)
    }
  } catch (e) {
    return new Response(JSON.stringify({error: e?.message ? e.message : e}), { status: 400, headers: headers});
  }
}
```

## Creating Client Applications
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

An example of creating an new client in the JavaScript demo dapp (that version has no LD format route):
```javascript
const witnessUrl = process.env.WITNESS_URL;
const statementUrl = `${witnessUrl}/statement`;
const instructionsUrl = `${witnessUrl}/instructions`;
const jwtUrl = `${witnessUrl}/witness`;

export const client = new Client(instructionsUrl, statementUrl, jwtUrl);
```

The advantage of this approach is that even if new Flows are added to the Witness SDK, clients should only need to update their version and recompile to have access to the new flows since the request and response type definitions have remained the same.

An example of usage of a WASM client is found in the [demo dapp codebase](https://github.com/spruceid/rebase/tree/main/demo/dapp), which can be seen deployed [here, at the Rebase credential faucet](https://rebase.pages.dev/).