# Rebase Dapp Demo

This is a thin UI around the [Rebase WASM Client](../../js/rebase-client/) for usage with the witness service found in [here](../../rust/rebase_cf_worker/), and it should work with any witness created using [the witness SDK](../../rust/rebase_witness_sdk). 

To run locally:
Navigate to this directory and then run the following:

```
$ npm i
$ npm run dev
```

The the UI should be running on `localhost:3000` and is simple to operate. Signers (currently limited to Ethereum and Solana) can be connected through the header and claims can be created by visiting the `available` option at the top of the app. 

In the future, there may be a more abstract way of handling witness flows, but for now, `WitnessForm` switches on the type and changes the instructions accordingly. The application also requries an `INFURA_ID` as an environment variable if you wish to utilize WalletConnect, otherwise it defaults to only supporting MetaMask.

By default, Phantom is the only supported Solana wallet available in the demo.
