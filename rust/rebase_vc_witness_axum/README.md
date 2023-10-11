# Rebase VC Witness Axum

This repository is a Witness and Credential Issuing service similar to [the Cloudflare Worker project found here](https://github.com/spruceid/rebase/tree/main/rust/rebase_cf_worker). It has the same external API but internally is an Axum service for easy integration into Rust Axum servers.

A dead-simple example is found in `examples/server_example.rs` and can be run via `cargo` in the normal way to run project examples.

Because it's written as an Axum Router, it can be used anywhere a Axum/Tower Service can, so it should be very flexible.

The example requires a `rebase.json` file to exist and it's contents must match the `example_rebase.json` file provided in structure, though much of the config is optional. See the source / the CF worker's source for more details.