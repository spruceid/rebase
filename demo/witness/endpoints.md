# Rebase Witness API

The witness API provides the generation of statements for cryptographically verifiable claims and the witnessing of these claims to produce [Verifiable Credentials](https://www.w3.org/TR/vc-data-model/). A concrete implementation of this API exists in the Cloudflare Worker found [here](https://github.com/spruceid/rebase/tree/main/demo/witness). It could also be implemented as a more traditional web server, or as a serverless function. Regardless, the witness exposes two routes, `/statement` and `/witness`. These routes are described in detail in the following sections. 

They exist in the expected context of a UI delivering information (including a cryptographic public key) from a user to the witness via `/statement` to create a statement, then the user signing the statement with public key described in the statement. After the statement is signed, and in some cases published, the user then gives the witness enough information to validate that the signature and statement match the statement's attestation through the `/witness` route. Assuming everything is valid, the witness issues a Verifiable Credential in the form of a [JWT](https://jwt.io/introduction).

## Issuance Flow

1) The user supplies information relevant to create a statement. It will always include at least one public key and either an identifier to link to that key, or a second public key to link with the first.

2) The information from step 1 is passed to the worker in the body of a `POST` request to the route `/statement`

3) The information is parsed, and if it matches the form specified below, is used to generate a plain text statement and a delimitor (sometimes unused) which are returned in the response body as `statement` and `delimitor` respectively.

4) The user signs the statement at least once. For linking two public keys, the statement is signed twice and signatures returned to the witness in the format as described below. In the case of other identity linking flows, a post is created in the format of: `${statement}${delimitor}${signature}`, then posted somewhere only the identity in the statement could access. Once posted, the location of this post, along with the options used to generate the statement initially are supplied to `/witness`.

5) The witness parses the `POST` request and assuming it conforms to the format described below either validates both signatures, or retrieves the post, compares the owner of the post to the owner described in the statement and verifies that public key described in the statement was the one that signed the statement. 

6) Presuming all of that is valid the witness returns a Verifiable Credential to the end user.

## Statement and Witness Requests

All responses that are not errors are consistent in their format as JSON objects. 

The `statement` response rendered as a typescript type is always:
```typescript
interface StatementRes {
    "statement": string,
    "delimitor": string
}
```
The `witness` response rendered as a typescript type is always:
```typescript
interface WitnessRes {
    "jwt": string,
}
```

The `POST` body of `/statement` must conform to a JSON representation of the supported Rebase `statement_type`, the source of which in Rust can be found [here](https://github.com/spruceid/rebase/blob/main/rust/rebase/src/witness/statement_type.rs).

The typescript definition of a Statement body would look like:
```typescript
interface Statement {
    opts: Statements;
}
```

With `Statements` being a sum type:
```typescript
type Statements =
    | DnsStmt
    | GitHubStmt
    | TwitterStmt
    | SelfSignedStmt;
```

And the individual statements being:
```typescript
interface DnsStmt {
    dns: {
        domain: string;
        prefix: string;
        key_type: KeyTypes;
    };
}

interface GitHubStmt {
    github: {
        handle: string;
        key_type: KeyTypes;
    };
}

interface TwitterStmt {
    twitter: {
        handle: string;
        key_type: KeyTypes;
    };
}

interface SelfSignedStmt {
    self_signed: {
        key_1: KeyTypes;
        key_2: KeyTypes;
    };
}
```
KeyTypes must conform to the JSON representation of the supported Rebase `DID` type found [here](https://github.com/spruceid/rebase/blob/main/rust/rebase/src/signer/signer.rs#L64).
They typescript definition of KeyTypes and it's child types would look like:
```typescript
type KeyTypes = Eth | Web;

interface Eth {
    pkh: {
        eip155: {
            address: string;
            chain_id: "1";
        };
    };
}

interface Web {
    web: string
}
```

A sample GitHub request would look like: 
```json
{
    "opts": {
        "github": {
            "handle": "foo",
            "key_type": {
                "pkh": {
                    "eip155": {
                        "address": "0x1111111111111111111111111111111111111111",
                        "chain_id": "1"
                    }
                }
            }
        }
    }
}
```

The response would look like:
```json
{
    "statement": "I am attesting that this GitHub handle foo is linked to the Ethereum Address 0x1111111111111111111111111111111111111111",
    "delimitor": "\n\n"
}
```

## Witness flow

The `POST` body of `/witness` must conform to a JSON representation of the supported Rebase `proof_type`, the source of which in Rust can be found [here](https://github.com/spruceid/rebase/blob/main/rust/rebase/src/witness/proof_type.rs).

The typescript definition of a Witness body would look like:
```typescript
interface Proof {
    proof: ProofTypes;
}
```
With `ProofTypes` being a sum type:
```typescript
type ProofTypes =
        | DnsStmt
        | GitHubProof
        | TwitterProof
        | SelfSignedProof;
```
(NOTE: DnsStmt is both used for the statement route and the witness route)

The other proofs look like:
```typescript
interface GitHubProof {
    github: {
        gist_id: string;
        statement_opts: GitHubStmt;
    };
}

interface TwitterProof {
    twitter: {
        statement_opts: TwitterStmt;
        tweet_url: string;
    };
}

interface SelfSignedProof {
    self_signed: {
        signature_1: string;
        signature_2: string;
        statement_opts: SelfSignedStmt;
    }
}
```

An example (VALID) request for a GitHub proof would look like:

```json
{
    "proof": {
        "github":{
            "statement_opts":{
                "handle":"krhoda",
                "key_type": {
                    "pkh": {
                        "eip155": {
                            "address":"0xdA3176d77c04632F2862B14E35bc6B4717FB5016","chain_id":"1"
                        }
                    }
                }
            },
            "gist_id":"3836dc1154d9499c55106c84b9c4bc1c"
        }
    }
}
```
The (VALID) result of such a request would look like:
```json
{
    "jwt": "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDp3ZWI6cmViYXNlZGVtb2tleS5wYWdlcy5kZXYjY29udHJvbGxlciJ9.eyJpc3MiOiJkaWQ6d2ViOnJlYmFzZWRlbW9rZXkucGFnZXMuZGV2IiwibmJmIjoxNjU1MzI0Mjc0LjE5NywianRpIjoidXJuOnV1aWQ6NTVjNGJjNzktZmFhMC00MDg2LWE1ZmEtMmFhZWYxYjA2ZDY0Iiwic3ViIjoiZGlkOnBraDplaXAxMTU6MToweGRBMzE3NmQ3N2MwNDYzMkYyODYyQjE0RTM1YmM2QjQ3MTdGQjUwMTYiLCJ2YyI6eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvMjAxOC9jcmVkZW50aWFscy92MSIseyJHaXRIdWJWZXJpZmljYXRpb24iOiJodHRwczovL2V4YW1wbGUuY29tL0dpdEh1YlZlcmlmaWNhdGlvbiIsIkdpdEh1YlZlcmlmaWNhdGlvbk1lc3NhZ2UiOnsiQGNvbnRleHQiOnsiQHByb3RlY3RlZCI6dHJ1ZSwiQHZlcnNpb24iOjEuMSwiZ2lzdElkIjoiaHR0cHM6Ly9leGFtcGxlLmNvbS9naXN0SWQiLCJoYW5kbGUiOiJodHRwczovL2V4YW1wbGUuY29tL2hhbmRsZSIsInRpbWVzdGFtcCI6eyJAaWQiOiJodHRwczovL2V4YW1wbGUuY29tL3RpbWVzdGFtcCIsIkB0eXBlIjoiaHR0cDovL3d3dy53My5vcmcvMjAwMS9YTUxTY2hlbWEjZGF0ZVRpbWUifX0sIkBpZCI6Imh0dHBzOi8vZXhhbXBsZS5jb20vR2l0SHViVmVyaWZpY2F0aW9uTWVzc2FnZSJ9LCJzYW1lQXMiOiJodHRwOi8vc2NoZW1hLm9yZy9zYW1lQXMifV0sImlkIjoidXJuOnV1aWQ6NTVjNGJjNzktZmFhMC00MDg2LWE1ZmEtMmFhZWYxYjA2ZDY0IiwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIkdpdEh1YlZlcmlmaWNhdGlvbiJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJpZCI6ImRpZDpwa2g6ZWlwMTE1OjE6MHhkQTMxNzZkNzdjMDQ2MzJGMjg2MkIxNEUzNWJjNkI0NzE3RkI1MDE2Iiwic2FtZUFzIjoiaHR0cHM6Ly9naXRodWIuY29tL2tyaG9kYSJ9LCJpc3N1ZXIiOiJkaWQ6d2ViOnJlYmFzZWRlbW9rZXkucGFnZXMuZGV2IiwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wNi0xNVQyMDoxNzo1NC4xOTdaIiwiZXZpZGVuY2UiOnsidHlwZSI6WyJHaXRIdWJWZXJpZmljYXRpb25NZXNzYWdlIl0sImhhbmRsZSI6ImtyaG9kYSIsInRpbWVzdGFtcCI6IjIwMjItMDYtMTVUMjA6MTc6NTQuMTk3WiIsImdpc3RJZCI6IjM4MzZkYzExNTRkOTQ5OWM1NTEwNmM4NGI5YzRiYzFjIn19fQ.ODuLC1uuJTaQ_buxyOtklw-XDEfGwBeaR14scLz4FvqTCQzbg2w4mtNgozVCoHJpAIqmgzRPMFynJMmOaM-9CA"
}
```

Once decoded, the JWT shows as:
The header:
```json
{
  "alg": "EdDSA",
  "kid": "did:web:rebasedemokey.pages.dev#controller"
}
```
The body:
```json
{
  "iss": "did:web:rebasedemokey.pages.dev",
  "nbf": 1655324274.197,
  "jti": "urn:uuid:55c4bc79-faa0-4086-a5fa-2aaef1b06d64",
  "sub": "did:pkh:eip155:1:0xdA3176d77c04632F2862B14E35bc6B4717FB5016",
  "vc": {
    "@context": [
      "https://www.w3.org/2018/credentials/v1",
      {
        "GitHubVerification": "https://example.com/GitHubVerification",
        "GitHubVerificationMessage": {
          "@context": {
            "@protected": true,
            "@version": 1.1,
            "gistId": "https://example.com/gistId",
            "handle": "https://example.com/handle",
            "timestamp": {
              "@id": "https://example.com/timestamp",
              "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
            }
          },
          "@id": "https://example.com/GitHubVerificationMessage"
        },
        "sameAs": "http://schema.org/sameAs"
      }
    ],
    "id": "urn:uuid:55c4bc79-faa0-4086-a5fa-2aaef1b06d64",
    "type": [
      "VerifiableCredential",
      "GitHubVerification"
    ],
    "credentialSubject": {
      "id": "did:pkh:eip155:1:0xdA3176d77c04632F2862B14E35bc6B4717FB5016",
      "sameAs": "https://github.com/krhoda"
    },
    "issuer": "did:web:rebasedemokey.pages.dev",
    "issuanceDate": "2022-06-15T20:17:54.197Z",
    "evidence": {
      "type": [
        "GitHubVerificationMessage"
      ],
      "handle": "krhoda",
      "timestamp": "2022-06-15T20:17:54.197Z",
      "gistId": "3836dc1154d9499c55106c84b9c4bc1c"
    }
  }
}
```

### Instructions Flow

An optional instructions flow is made avaiable through a `POST` route using `/instructions`. The body of this request would be defined in Typescript like so:

```typescript
interface InstructionsReq {
    type: InstructionsType
}

type InstructionsType = "dns" | "github" | "self_signed" | "twitter"
```

The response is defined in Typescript as:
(Where JSONSchema is a valid JSONSchema)

```typescript
interface InstructionsRes {
    instructions: Instructions,
    statement_schema: JSONSchema,
    witness_schema: JSONSchema
}

interface Instructions {
    statement: string,
    signature: string,
    witness: string
}
```

This response provides several useful features, first a set of text instructions that could be used to automatically generate UI flows corresponding to the requested flow type. Secondly, a pair of JSONSchemas, which allow for at minimum checking the outgoing `POST` bodies for formatting errors, or in the extereme, generating forms that correspond to the witness flow. If used to their maximum, one could simply generate the entire UI flows dynamically.