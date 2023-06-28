To run, first run the `rebase_cf_worker` found in `../../rust/rebase_cf_worker` locally using `wrangler dev -r`. Once it's running at localhost:8787, then run this repo via the commands: `npm i && npm run dev`.

This is a basic example app for demonstrating delegated attestation flows and how session keys are used to enable them. 

It will be updated to use the live CloudFlare worker rather than a local copy in the near future.