# Examples

## TODO: ToC

## Introduction

Rebase is a library for working with [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/). Many crpytographic keys used to sign Verifiable Credentials are tied to cryptocurrencies, so mocking them for the purposes of examples can be tricky. For this reason, we're going to stick to ed25519 keys. This is still a useful endevour because such keys are used in "witness" services, a key piece of Rebase architecture.

Once the WASM portions of the library are complete, look forward to more robust, easier to try examples involving crypto wallets!

## General Requirements

These examples assume you have [cargo](https://www.rust-lang.org/tools/install) and [didkit-cli](https://github.com/spruceid/didkit/tree/main/cli) installed and on path. 

You will also need a basic web file server (`npx http-server`, `python3 -m http.server`, etc will work fine). 

Finally, you'll need a way to point at that server without refering to the port (why that is so will be covered shortly) and over HTTPS. Either set up your snake-oil certs and give permission to use port 443 or just use [ngrok](https://ngrok.com/download). If you don't know what that any of that means, that's fine, the instructions will assume you're using the free, account-less tier of ngrok and walk you through it.

## Background

Because Rebase is built on top of [didkit](https://spruceid.dev/docs/didkit/) the credentials it creates are [DID documents](https://www.w3.org/TR/did-core/) which happen to be [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/). There are many type of [DID methods](https://www.w3.org/TR/did-spec-registries/) used to resolve the veracity of compliant credentials, ranging from those based on cryptocurrency wallet signatures to more abstract approaches like `did:web` used in these examples. To allow users to quickly try Rebase these examples use [did:web](https://spruceid.dev/docs/didkit/did-web/) methods and [ed25519](https://ed25519.cr.yp.to/) keys that the user generates using `didkit`. 

`did:web` is a resolution method that allows the consumer of a Verifiable Credential to look up the corresponding public key through an http request. It is described in detail [here](https://spruceid.dev/docs/didkit/did-web/). We will be setting up a local server to make a public key available for `did:web` resolution, then using the corresponding private key to issue credentials. These credentials will be verifiable so long as the server which `did:web` resolves to is available.

## Basic

### Running the example

This example showcases the most basic type of claim. This claim simply asserts that it was signed by a particular key, implying ownership or affirmation by the controller of that key. 

An example usecase of this type of claim could be social media posts. The authorship of the posts becomes independently verifiable and completely portable. 

To run this example, we will create a key capable of signing Verifiable Credentials and a small, local webserver to run the `did:web` resolution. At the end, we will have self-signed Verifiable Credential that we will check with `didkit`.

First, from this `examples` directory, run

```bash
./ed25519_basic_setup.sh
```

This simple script will create a `./temp/ed25519_basic` directory and populate it with a couple of neccessary files and directories:

`keys/controller.jwk`: The [JWK](https://datatracker.ietf.org/doc/html/rfc7517) format of the ed25519 key pair to be used to create and resolve credentials.

`.well-known/did.json`: The [DID document](https://www.w3.org/TR/did-core/) output from `controller.jwk`, described more fully in the [did:web help document](https://spruceid.dev/docs/didkit/did-web/). This file includes the public key found in `controller.jwk`, is what will be served by the file server when someone resolves the created credential. It's placed in the `.well-known` directory because we'll run the webserver from `./temp/ed25519_basic` and `did:web` normally looks for this pattern.

`credentials`: This directory will be populated by running the actual Rust example.

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
(NOTE: the URL will be different for your own instance of ngrok)

The URL (in this example `https://72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io`) is key. 

Now, leaving `ngrok` and our web server still running, from `rebase/rust/rebase`, one directory higher than this document, we can run:

```bash
cargo run --example ed25519_basic -- 72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io
```
(NOTE: the URL will be different for your own instance of ngrok)

You will be prompted to enter a title and body for a "post", which will be the contents embedded in the credential. It isn't actually being posted anywhere, just saved to the local file system. 

In a production system, this could be the creation of a post or event.

Once the infromation is gathered, the example will produce a Verifiable Credential using the key generated by the shell script and the text proveded by the user at the CLI.

The terminal will print the issued credential and also write it to `./examples/temp/ed25519_basic/credentials/vc.json`. As long as our web server and ngrok service are running we can verify the credential with didkit. From where we ran the `cargo` example (`rebase/rust/rebase`) we can run:

```bash
didkit vc-verify-credential \ 
-v did:web:72a8-2601-285-8280-60d0-94f1-6502-1176-cd2f.ngrok.io#controller \
-p assertionMethod \ 
< ./examples/temp/ed25519_basic/credentials/vc.json
```

(NOTE: the URL will be different for your own instance of `ngrok`)

The result should be:
```bash
{"checks":["proof"],"warnings":[],"errors":[]}%
```

This demonstrates that the issued credential can be independently verified. In production settings, the `did:web:<URL>`'s `<URL>` would the `https` address of a publically avaiable webserver, allowing anyone, anywhere with an internet connection to trustlessly verify the credential was signed by the expected key. Pretty cool, but how does it work underneath?

### What the example does.

Let's step through the shell script then the rust script step by step.

Lines 1-12 of the shell script simply check for dependencies and create needed folders. The last two lines are the only really interesting ones. First:
```bash
didkit generate-ed25519-key >> ./keys/controller.jwk
```

This `didkit` command creates a `ed25519` key pair as described in the links above. This gives a key that is compatable with `rebase`'s `Signer` abstraction and lets us sign any of the supported `rebase` credentials. In otherwords, this lets us create credentials. The next step will let others read our credentials:
```bash
didkit did-resolve `didkit key-to-did key -k ./keys/controller.jwk` >> ./.well-known/did.json
```

This `didkit` command takes the key pair generated in the line above and creates a `did` document from it. This `did` document is used in `did:web` resolution so that a consumer of our credential can locate the public key material corresponding to our credential. Once they have the credential and public key, the consumer can verify the credential was signed by the key it purports.

The script drops the `did` document in the `.well-known` folder so that it can be easily served by a simple web server.

Once the key and `did` document are in place, the example rust code can be run. The `main` function looks like so:

```rust
async fn main() {
    let url = env::args().skip(1).next().unwrap();
    let id = format!("did:web:{}", &url);

    let key = get_key().unwrap();
    fmt_did(url).unwrap();

    let signer = rebase::signer::ed25519::Ed25519::new(
        id,
        key,
        "controller".to_string(),
        rebase::signer::ed25519::SignerTypes::DIDWebJWK,
    )
    .await
    .unwrap();

    println!("Let's make a post, then save it out as a Verifiable Credential!");
    println!("Enter the title of your post:");

    let title = get_line().unwrap();

    println!("Good, now for the body of the post:");

    let body = get_line().unwrap();

    let schema = rebase::schema::basic_post::BasicPost { title, body };

    let credential = schema.credential(&signer).await.unwrap();
    let s = to_string(&credential).unwrap();

    println!("{}", s);
    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("./examples/temp/ed25519_basic/credentials/vc.json")
        .unwrap();

    f.write_all(s.as_bytes()).unwrap()
}
```

A lot of `unwrap`s because it's an example, but other than that, hopefully pretty straight forward. The first three actions of the function are:
```rust
let url = env::args().skip(1).next().unwrap();
let id = format!("did:web:{}", &url);

let key = get_key().unwrap();
```

We get the url passed in by the caller, which will be used for did resolution and we build a `did:web` identifier using it, then we open the key file from the file system and serialize it into the [ssi](https://github.com/spruceid/ssi/blob/main/src/jwk.rs) library's [JWK representation](https://openid.net/specs/draft-jones-json-web-key-03.html) using the local `get_key` function.

The next one is a little more interesting:

```rust
fmt_did(url).unwrap();
```

`fmt_did` opens up the `did` document file on the file system (`./examples/temp/ed25519_basic/.well-known/did.json`) and edits it so that it will properly resolve using the current `ngrok` url. In a production system the `did:web:<URL>`'s `<URL>` would be static, so this step would be unneeded, but because we want this example to be runnable without a public webserver, such book-keeping is needed.

The difference between the initial file created and what the example produces looks like this:

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

The next block of code sets up the `rebase::signer::Signer` abstraction:
```rust
let signer = rebase::signer::ed25519::Ed25519::new(
    id,
    key,
    "controller".to_string(),
    rebase::signer::ed25519::SignerTypes::DIDWebJWK,
)
.await
.unwrap();
```

After this point, the example script doesn't need to know anything about the signer to work. Other keys could be substituted. Once a signer is created, it can be used with any `rebase` schema to produce credentials.

The next several lines are about getting user input from the terminal in order to build the credential contents:
```rust
println!("Let's make a post, then save it out as a Verifiable Credential!");
println!("Enter the title of your post:");

let title = get_line().unwrap();

println!("Good, now for the body of the post:");

let body = get_line().unwrap();
```

Once the information from the user has been gathered, we can create a BasicPost schema:

```rust
let schema = rebase::schema::basic_post::BasicPost { title, body };
```

Then we can sign it:

```rust
let credential = schema.credential(&signer).await.unwrap();
```

Again, any Rebase `schema` can use any Rebase `signer` to create credentials. In this case, we created a Credential from a BasicPost schema and a Ed25519 key pair. The Ed25519 `Signer` defines that it uses `did:web` for resolution and is able to handle the plumbing between itself and `didkit`.

The remaining lines are just printing the credential and saving it out.

Now that the most basic example is out the way, let's move to Credentials that say more than just someone endorsed their contents.

## Crosskey Example

# TODO: Implement