## Overview

This document describes the needed secrets to be set in order to operate the with CloudFlare Witness/Issuer service contained here. These secrets are most easily set using [wrangler](https://developers.cloudflare.com/workers/wrangler/get-started/) via the process [described here](https://blog.cloudflare.com/workers-secrets-environment/).

## IMPORTANT NOTE!

All secrets must be set, but setting them to an empty string will cause them to be passed over in configuration. 

## Required for All Flows:
### REBASE_SK
The `REBASE_SK` secret should be the JSON stringified version of an ED25519 JWK Secret Key. Such a key can be generated following the [instructions given here](https://www.spruceid.dev/didkit/didkit-examples/did-web-in-minutes).

### DID_WEB
The `DID_WEB` secret should be a [did:web](https://w3c-ccg.github.io/did-method-web/) identifier following the format `did:web:<URL_HOST_OF_REBASE_PUBLIC_KEY>`. This corresponds directly to `REBASE_SK` being the public key DID document that pairs with the secret JWK. 

For example, if the`DID_WEB` were set to `did:web:example.com`, it would be expected that visiting `example.com/.well-known/did.json` would resolve to a DID Web JSON object usable with `REBASE_SK`'s JWK. 

The [instructions linked earlier](https://www.spruceid.dev/didkit/didkit-examples/did-web-in-minutes) show how to create this document after creating the secret key JWK.

## Required for Certain Flows

### GITHUB_USER_AGENT (GitHub Flow)
#### Required if using GitHub flow.
The `GITHUB_USER_AGENT` secret will be the user agent sent to GitHub when querying it's public API.

### TWITTER_BEARER_TOKEN (Twitter Flow)
#### Required if using Twitter flow.
The `TWITTER_BEARER_TOKEN` is the bearer token given from Twitter to the application developer using the [Twitter API](https://developer.twitter.com/en/docs/twitter-api) and will be used (as described [here](https://developer.twitter.com/en/docs/authentication/oauth-2-0/bearer-tokens)) when querying the API.

### SOUNDCLOUD_CLIENT_ID (SoundCloud Flow)
#### Required if using SoundCloud flow.
The `SOUNDCLOUD_CLIENT_ID` secret is the client id used in the SoundCloud v2 API, this can be discovered by any logged in SoundCloud user following the directions [outlined here](https://stackoverflow.com/a/54174507).

### SOUNDCLOUD_LIMIT (SoundCloud Flow)
#### Required if using SoundCloud flow.
The `SOUNDCLOUD_LIMIT` secret is the number of search results returned per query of the SoundCloud user search. Must be between an integer between 0 and 201 (exclusive, inclusive 1 through 200). Suggested to be set to 100.

### SOUNDCLOUD_MAX_OFFSET (SoundCloud Flow)
#### Required if using SoundCloud flow.
The `SOUNDCLOUD_MAX_OFFSET` number of search results to try before giving up (the total tested will be `SOUNDCLOUD_MAX_OFFSET` + `SOUNDCLOUD_LIMIT`, since it starts at 0). `SOUNDCLOUD_MAX_OFFSET` + `SOUNDCLOUD_LIMIT` must be less than or equal to 10k. Suggested to be set to 900.