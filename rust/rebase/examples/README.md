# Examples

## TODO: ToC

## Introduction

Rebase is a library for dealing with [Verifiable Credentials](), which are cryptographically verifiable. Most crpytographic keys in use are tied to currencies, so mocking them for the purposes of cli examples can be tricky. For this reason, we're going to stick to ed25519 keys and signing for example purposes. This is still a useful endevour because such keys can be used in "witness" services, a key piece of Rebase architecture.

Once the WASM portions of the library are complete, look forward to more robust, easier to try examples involving crypto wallets!

## General Requirements

These examples assume you have [cargo](https://www.rust-lang.org/tools/install) and [didkit-cli](https://github.com/spruceid/didkit/tree/main/cli) installed and on path. 

You will also need a basic web file server (`npx http-server`, `python3 -m http.server`, etc will work fine). 

Finally, you'll need a way to point at that server without refering to the port (why that is so will be covered shortly) and over HTTPS. Either set up your snake-oil certs and give permission to use port 443 or just use [ngrok](https://ngrok.com/download). If you don't know what that any of that means, that's fine, the instructions will assume you're using the free, account-less tier of ngrok and walk you through it.

## Background

Because Rebase is built on top of [didkit]() the credentials it creates are [DID documents]() which happen to be [Verifiable Credentials](). There are many type of [DID methods]() used to resolve the veracity of compliant credentials, ranging from those based on cryptocurrency wallet signatures to more abstract approaches like `did:web` used in these examples. To allow users to quickly try Rebase these examples use [did:web](https://spruceid.dev/docs/didkit/did-web/) methods and [ed25519]() keys that the user generates using `didkit`. 

`did:web` is a resolution method that allows the consumer of a Verifiable Credential to look up the corresponding public key through an http request. It is described in detail [here](https://spruceid.dev/docs/didkit/did-web/). We will be setting up a local server to make a public key available for `did:web` resolution, then using the corresponding private key to issue credentials. These credentials will be verifiable so long as the server which `did:web` resolves to is available.

## Basic

First, from this `examples` directory, run

```bash
./ed25519_basic_setup.sh
```

This simple script will create a `./temp/ed25519_basic` directory and populate it with a couple of neccessary files and directories:

`keys/controller.jwk`: The [JWK]() format of the ed25519 key pair to be used to create and resolve credentials.

`.well-known/did.json`: The [DID document]() output from `controller.jwk`, described more fully in the [did:web help document](https://spruceid.dev/docs/didkit/did-web/). This file includes the public key found in `controller.jwk`, is what will be served by the file server when someone resolves the created credential. It's placed in the `.well-known` directory because we'll run the webserver from `./temp/ed25519_basic` and `did:web` normally looks for this pattern.

Second, we'll set up the web server. In this example, we'll use a python server, but any local file server would work.

```bash
cd temp/ed25519_basic
python3 -m http.server
```

Third, in a new terminal, we will run ngrok in order to get a fully qualified, SSL url that we can use in the `did` documents.

```bash
ngrok http 8000
```
(NOTE: Replace `8000` with whatever port the server from is listening on)

The result of the above command should include a line that looks like:

```
Forwarding                    https://72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io -> http://localhost:8000
```

The URL (in this example `https://72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io`) is key. As alluded to in the (did:web document)[], several fields need to be changed in `.well-known/did.json` to use this URL. Using the example URL an edit would look something like:
```diff
{
  "@context": [
    "https://www.w3.org/ns/did/v1",
    {
      "Ed25519VerificationKey2018": "https://w3id.org/security#Ed25519VerificationKey2018",
      "publicKeyJwk": {
        "@id": "https://w3id.org/security#publicKeyJwk",
        "@type": "@json"
      }
    }
  ],
-  "id": "did:key:z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf",
+  "id": "did:web:72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io",
  "verificationMethod": [
    {
-      "id": "did:key:z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf#z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf",
+      "id": "did:web:72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io#controller",
      "type": "Ed25519VerificationKey2018",
-      "controller": "did:key:z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf",
+      "controller": "did:web:72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io",
      "publicKeyJwk": {
        "kty": "OKP",
        "crv": "Ed25519",
        "x": "e3QlvNPlFtj7_9hSU_s8pM44rbMIbr3PvKSgCfSvD04"
      }
    }
  ],
  "authentication": [
-    "did:key:z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf#z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf"
+    "did:web:72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io#controller"
  ],
  "assertionMethod": [
-    "did:key:z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf#z6MknmAuwpaLVrYGTYCtodTj6KoitVNxGz5wzAwHNmEQYcsf"
+    "did:web:72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io#controller"
  ]
}
```

This is the reason we need `ngrok`, to meet the URL requirements of `did` documents and avoid specifying a port. There are other ways around this problem, but this works for an example.

Now, leaving `ngrok` and our web server still running, from `rebase/rust/rebase`, one directory higher than this document, we can run:

```bash
cargo run --example ed25519_basic -- 72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io
```

The terminal will print the issued credential.

TODO: Add outputting the Credential to a file and show verifying it with didkit.