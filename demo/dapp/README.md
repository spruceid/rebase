# Rebase Dapp Demo

This is a thin client for usage with the witness service found in `rebase/demo/witness`. Running locally should be as simple as `cd`ing to this directory then:

```
$ npm i
$ npm run dev
```

Then the UI should be running on `localhost:3000` and simple to operate. Signers (currently limited to Ethereum) can be connected through the header and claims can be created by visiting the `available` option at the top of the body.

The witness flows should contain sufficient instructions for a user to follow. If using the default configuration, `rebase/demo/witness` will need to be running on `localhost:8787`. If using a different witness, change the const `witnessUrl` in `demo/dapp/src/components/claims/WitnessForm.svelte` to the location of the witness you want to use.

In the future, there may be a more abstract way of handling witness flows, but for now, `WitnessForm` switches on the type and changes the instructions accordingly. Also requries a `INFURA_ID` as an environment variable if wanting to utilized WalletConnect, otherwise defaults to only supporting MetaMask.
