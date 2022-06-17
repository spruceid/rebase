# Rebase Witness

A Cloudflare Worker to act as a witness for Rebase claims.

## Deploy

Requires [Wrangler](https://developers.cloudflare.com/workers/cli-wrangler/install-update#additional-installation-instructions).

You need a Cloudflare account, and the Account ID needs to go in
the `wrangler.toml`.

The worker needs two things: the private key to issue the VC after verifying the
tweet, and an API token for the relevant social APIs. You can add these as secrets with:

```bash
wrangler secret put REBASE_SK
wrangler secret put TWITTER_BEARER_TOKEN
```

> The private key is expected to be a JWK. You can generate one with
> `didkit generate-ed25519-key`.

> This key is also expected to have a corresponding `did:web` outlined

```bash
wrangler publish
```

> For development, you should use `wrangler dev`. This will launch the worker to listen on `localhost:8787`.

Regardless of where it's deployed, the worker responds to two routes `/statement` and `/witness`, both expect a POST. The former expects the `POST` body to conform to:
```rust
#[derive(Deserialize, Serialize)]
pub struct StatementReq {
    pub opts: StatementTypes,
}
```
for Statement and for Witness:
```rust
#[derive(Deserialize, Serialize)]
pub struct WitnessReq {
    pub proof: ProofTypes,
}
```

Details on `StatementTypes` and `ProofTypes` can be found in the top-level README and their implementations can be found in `rebase/rust/src`.