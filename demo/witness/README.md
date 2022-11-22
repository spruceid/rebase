# Rebase Witness

A Cloudflare Worker to act as a witness for Rebase claims and an issuer of Rebase credentials.

## Deploy

Requires 
* [Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update#additional-installation-instructions).
* A Cloudflare account (the Account ID needs to go in the `wrangler.toml`).
* Several secrets for the Worker.

The Worker needs several secrets set using `wrangler secret put`, the details of which are outlined in this repository's `secrets.md` [found here](https://github.com/spruceid/rebase/blob/main/demo/witness/secrets.md). Additionally, the `wrangler.toml` must be exist and contain the properties found in the `wrangler.example.toml`.

To publish a new version of the worker to your CloudFlare account simply run:
```bash
wrangler publish
```

> For development, you should use `wrangler dev`. This will launch the worker to listen on `localhost:8787`.

Regardless of where it's deployed, the worker responds to three routes `/instructions`, `/statement` and `/witness`, each expect a POST request. They respectively expect the `POST` body to conform to:
```rust
#[derive(Deserialize, Serialize)]
pub struct InstructionReq {
    #[serde(rename = "type")]
    pub instruction_type: InstructionTypes,
}

#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    pub opts: Statements,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: Proofs,
}
```

Details on `InstructionTypes`, `Statements` and `Proofs` can be found in the [Rebase Witness SDK repository](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk). 

More information about the structure of the responses to these requests can be found in the [endpoints.md document](https://github.com/spruceid/rebase/tree/main/demo/witness/endpoints.md).
