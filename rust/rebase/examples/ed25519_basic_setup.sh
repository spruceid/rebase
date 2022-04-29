#!/bin/bash

if ! command -v didkit &> /dev/null
then
    echo "Please install didkit"
    exit
fi

mkdir -p ./temp/ed25519_basic/.well-known
cd ./temp/ed25519_basic
mkdir -p keys
mkdir -p credentials
didkit generate-ed25519-key >> ./keys/controller.jwk
didkit did-resolve `didkit key-to-did key -k ./keys/controller.jwk` >> ./.well-known/did.json
