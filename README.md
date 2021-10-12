# rebase

Rebase is a set of tools for encoding cross-domain, portable identity claims (in
particular, “verifications” like proof of control, history, and reputation
claims) as verifiable credentials to enable user-centric identity systems across
domains. 

The core of this toolset is a claims engine powered by verifiable credentials
(rebase-server), and a storage/query engine for publishing them in a way that
can be storage-agnostic, hash-protected, interoperable, and portable
(rebase-client).  The latter is described in the [/sdk/README.md](sdk/).

## Known Implementations

The first implementation of a Rebase system is **Tezos Profiles**, an engine for
creating, publishing to the Tezos blockchain and test-nets, querying, and
interpreting "profiles" that combine cross-domain identity verifications.  You
can interact with [the site](https://tzprofiles.com) yourself (requires
[Temple](https://templewallet.com/) or another Tezos wallet to create a
profile), watch the 10-minute [demo video](https://youtu.be/Ulfw332_-js), or go
straight to the [code](https://github.com/spruceid/tzprofiles).