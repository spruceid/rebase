if ! command -v didkit &> /dev/null
then
    echo "Please install didkit"
    exit
fi

mkdir -p ./temp/ed25519_crosskey/serve/key1/.well-known
cd ./temp/ed25519_crosskey
mkdir -p ./serve/key2/.well-known
mkdir -p keys
mkdir -p credentials
didkit generate-ed25519-key >> ./keys/key1.jwk
didkit generate-ed25519-key >> ./keys/key2.jwk
didkit did-resolve `didkit key-to-did key -k ./keys/key1.jwk` >> ./serve/key1/.well-known/did.json
didkit did-resolve `didkit key-to-did key -k ./keys/key2.jwk` >> ./serve/key2/.well-known/did.json
