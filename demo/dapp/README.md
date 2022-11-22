# Rebase Dapp Demo

This is a thin client for usage with the witness service found in `rebase/demo/witness`, and it should work with any witness created using [the witness SDK](https://github.com/spruceid/rebase/tree/main/rust/rebase_witness_sdk). 

To run locally:
* Navigate to this directory 
* Create a `WITNESS_URL` setting in a `.env` file (using either a publically deployed witness with a setting like `WITNESS_URL="https://rebasedemo.spruceid.workers.dev"`, or by running your own from this [repo](https://github.com/spruceid/rebase/tree/main/demo/witness), where the setting would likely be: `WITNESS_URL="http://localhost:8787"`) 

Then run the following:

```
$ npm i
$ npm run dev
```

The the UI should be running on `localhost:3000` and is simple to operate. Signers (currently limited to Ethereum and Solana) can be connected through the header and claims can be created by visiting the `available` option at the top of the app. 

In the future, there may be a more abstract way of handling witness flows, but for now, `WitnessForm` switches on the type and changes the instructions accordingly. The application also requries an `INFURA_ID` as an environment variable if you wish to utilize WalletConnect, otherwise it defaults to only supporting MetaMask.

By default, Phantom is the only supported Solana wallet available in the demo.
