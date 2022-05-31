# rebase

Rebase is a library built on top of DIDKit for creating certain types of Verifiable Credentials from schemas (eventually TreeLDR). Contains a plugin system to extend signing methods (initially supporting Ethereum, Tezos, and DIDWeb) and concrete implementations of the supported credential flows. 

Rebase is written in Rust with WASM as a first-class compilation target allowing for usage in the browser, server, and any platform WASM is supported.

Rebase uses schemas to define supported claims and maintain consistency in the Verifiable Credentials it produces. These schemas are used to meet the specification and can be used to generate basic HTML pages for LD schemes.

Schemas can represent many types of data, from publically witnessed attestations to cryptographically verifiable statements. As long as the schema implements the SchemaType trait, Rebase can manage it (TODO: Add examples or link to example here).

Schemas are combined with signers to create concrete VC creation and validation flows. Schemas describe what a VC represents. Signers describe how to create and check existing VCs. As new schemas are added they can be used with all existing signers and vice versa.

This library is useful for constructing clients based on Verifiable Credentials and witnesses for public attestation (TODO: Add examples or link to examples here)

Initially, Rebase will support three types of Schema / Credential Flows:

# TODO: Links to examples of each flow.

1) Self attested: These schemas are an attestation made by the owner of a particular key. They are self-signed, so the only guarantee is that the owner penned the statement, not that the statement is true. This is still useful for things like social media posts. The basic profile credential in TZProfiles is an example of another use case.

2) Cross Key: This schema takes two Signers (of the same or different type(s)) and creates a credential out of a signed statement attesting to shared ownership of the two signers. This statement is embedded in a credential signed by the other corresponding key. This allows for self-soveriegn linking of indentity between keys.

3) Publically Witnessed: This describes a flow where a user signs a statement with a Signer, then posts the statement and signature somewhere viewable by a "witness". The witness is then given the location of the post and the public key of the user's Signer. The witness looks up the post, validates the statement and signature correspond to the user's Signer, then using the witness' Signer, creates a credential attesting to the post. This allows linking of profiles to key ownership.

## TODO: Add sample code here.
# Potential File Structure

`rebase/schema`: Schemas of LD credentials used by Rebase libraries. One day will be TreeLDR -> Structs.

`rebase/rust/rebase`: Source code libraries Rebase exposes for calling applications to use.

`rebase/npm`: NPM modules wrapping the WASM output of the `rust` libraries.