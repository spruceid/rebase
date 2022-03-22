# rebase

Rebase is a library built on top of DIDKit for creating certain types of Verifiable Credentials from schemas (eventually TreeLDR). Contains a plugin system to extend signing methods (initially supporting Ethereum, Tezos, and DIDWeb) and concrete implementations of the supported credential flows. 

Rebase is written in Rust with WASM as a first-class compilation target allowing for usage in the browser, server, and any platform WASM is supported.

Rebase uses schemas to define supported claims and maintain consistency in the Verifiable Credentials it produces. These schemas are used to meet the specification and can be used to generate basic HTML pages for LD schemes.

These schemas define a `subject` and optionally a `proof` that are used to contstruct the Verifiable Credential in a given flow. These appear in the `@context` of the output Verifiable Credential but are also used for flow specific validation.

The common use of this library is to create either clients or witness services used to link identities between providers in a public, self-soveriegn way.

# Potential File Structure
## Open to change

`rebase/schema`: Schemas of LD credentials used by Rebase libraries. One day will be TreeLDR -> Structs.

`rebase/rust/rebase`: Source code libraries Rebase exposes for calling applications to use.

`rebase/npm`: NPM modules wrapping the WASM output of the `rust` libraries.

`rebase/sdk`: An initial, historical approach to Rebase. Will be divided and moved into `rebase/lib` as needed.