# rebase

Rebase is a library built on top of DIDKit for creating certain types of Verifiable Credentials from schemas (eventually TreeLDR). Contains a plugin system to extend signing methods (initially supporting Ethereum, Tezos, and DIDWeb) and concrete implementations of the supported credential flows. 

Rebase is written in Rust with WASM as a first-class compilation target allowing for usage in the browser, server, and any platform WASM is supported.

Rebase uses schemas to define supported claims and maintain consistency in the Verifiable Credentials it produces. These schemas are used to meet the specification and can be used to generate basic HTML pages for LD schemes.

These schemas define a `subject` and optionally a `proof` that are used to contstruct the Verifiable Credential in a given flow. These appear in the `@context` of the output Verifiable Credential but are also used for flow specific validation.

The common use of this library is to create either clients or witness services used to link identities between providers in a public, self-soveriegn way.

TODO: EXPAND!

3 Types of Schema / Credential Flows:
1) Self attested: This schema has no `proof`, it's just an attestation made by the owner of a particular key. The basic profile in TZProfiles is an example.

2) Cross Key: This flow generates it's `proof` from it's `subject`, so while the `proof` is present in the schema, the user will only pass a `subject`. It takes two `signer`s, generates a statement attesting to the linking of the two `signer`s, signs the statement with each `signer`, then embeds that signed statment into a VC signed by the other. The result is two cross signed VCs. This becomes interesting when paired with Kepler for purposes of discovery.

3) Publically Witnessed: This flow requires the user to present both a `subject` and `proof`. The flow has been abstracted so that the user provides a `subject` and `signer` for a `post`, then the user publically posts the `post`, getting a `post_location`. Then, by providing the `subject` and `post_location` to a public witness using this library, the public witness can verify the claim and issue a VC.

# Psuedo Code:

```
VC: didkit::Credential
Subject: Subject<Schema> where Schema: a_type_backed_by_schema
Proof: Proof<Schema> where Schema: a_type_backed_by_schema

// Didkit:
didkitify(Prep, Signer) -> Result<VC>

// Universal
new_prep(Subject, Option<Proof>) -> Result<Prep>

// Self-attestion
self_attest(Signer, Subject) -> Result<VC>

// Cross-key
cross_key(Signer, Signer, Subject) -> Result<VC>

// Publically Witnessed
to_public_claim(Subject, Signer) -> String
retrieve_claim(PostLocation, Subject) -> Result<Proof>
witness_claim(PostLocation, Subject, Sigher) -> Reuslt<VC> {
    didkitify(new_prep(subject, retrieve_claim(PostLocation, Subject)?)?, Signer)
}
```
# Potential File Structure
## Open to change

`rebase/schema`: Schemas of LD credentials used by Rebase libraries. One day will be TreeLDR -> Structs.

`rebase/rust/rebase`: Source code libraries Rebase exposes for calling applications to use.

`rebase/npm`: NPM modules wrapping the WASM output of the `rust` libraries.

`rebase/sdk`: An initial, historical approach to Rebase. Will be divided and moved into `rebase/lib` as needed.