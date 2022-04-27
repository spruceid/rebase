# Examples

## TODO: ToC

## Introduction

Rebase is a library for dealing with cryptographically verifiable credentials. Most crpytographic keys people make use of are tied to currencies, so mocking them for the purposes of examples is tricky. For this reason, we're going to stick to ed25519 keys and signing for example purposes. This is still a useful endevour because such keys can be used in "witness" services, a key piece of Rebase architecture.

Once the WASM portions of the library are complete, look forward to more robust, easier to try examples!

## General Requirements

These examples assume you have [cargo](https://www.rust-lang.org/tools/install) and [didkit-cli](https://github.com/spruceid/didkit/tree/main/cli) installed and on path. 

You will also need a basic web file server (`npx http-server` or `python3 -m http.server` etc will work fine). 

Finally, you'll need a way to point at that server without refering to the port (why that is so will be covered shortly) and over HTTPS. Either set up your snake-oil certs and give permission to use port 443 or just use [ngrok](https://ngrok.com/download). If you don't know what that any of that means, that's fine, the instructions will assume you're using the free tier of ngrok and walk you through it.

## Background

## Basic

