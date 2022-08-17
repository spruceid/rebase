# Rebase Witness

A Cloudflare Worker to act as a witness for Rebase claims.

## Deploy

Requires [Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update#additional-installation-instructions).

You need a Cloudflare account, and the Account ID needs to go in the `wrangler.toml`.

The worker needs several secrets set using `wrangler secret put`, the details of which are outlined in this repository's `secrets.md` [found here]().

To publish a new version of the worker to your CloudFlare account simply run:
```bash
wrangler publish
```

> For development, you should use `wrangler dev`. This will launch the worker to listen on `localhost:8787`.

Regardless of where it's deployed, the worker responds to three routes `/instructions`, `/statement` and `/witness`, each expect a POST. The former expects the `POST` body to conform to:
```rust
#[derive(Deserialize, Serialize)]
pub struct InstructionReq {
    #[serde(rename = "type")]
    pub instruction_type: InstructionTypes,
}

#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    pub opts: StatementTypes,
}

#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: ProofTypes,
}
```

Details on `InstructionTypes`, `StatementTypes` and `ProofTypes` can be found in the top-level README and their implementations can be found in `rebase/rust/src`.