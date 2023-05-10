#!/bin/bash
# Check for an -h flag.
while getopts 'lha:' OPTION; do
    case "$OPTION" in
        h)
            echo "This script is used to generate the typescript bindings from the underlying Rust libraries to augment wasm-pack's default build. It requires cargo and wasm-pack installed and on PATH. It will search for the rebase and rebase_witness_sdk at the relative path (from the location of this script, not from where it is invoked) '../../rust/<target_repo>' but can be set as positional parameters respectively. Please no trailing slashes! Scope can be overridden with the 3rd positional parameter, but at that point, you've probably forked this, you might as well just edit it!"
            exit 1
            ;;
    esac
done
shift "$(($OPTIND -1))"

# Set these with either a positional parameter or default to the path it would be if working in the monorepo.
# TODO: Tolerate trailing slashes by checking for, then stripping them.
REBASE_PATH=${1:-"../../rust/rebase"}
REBASE_WITNESS_PATH=${2:-"../../rust/rebase_witness_sdk"}
SCOPE=${3:-"spruceid"}

# This is an StackOverflow trick, I will not lie.
# Feel free to replace with something more eloquent/robust.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Sanity check.

if [ ! -f "${SCRIPT_DIR}/binding_glue/manual/index.ts" ]
then
    echo "No index.ts overwrite file found at $SCRIPT_DIR/binding_glue/manual/index.ts, it's needed to re-export the type bindings to consuming applications"
    exit
fi

if [ ! -d "${SCRIPT_DIR}/binding_glue/autogenerated" ]
then
    mkdir "${SCRIPT_DIR}/binding_glue/autogenerated" && echo "Found glue code"
fi

if [ -d "${SCRIPT_DIR}/binding_glue/autogenerated/tmp" ]
then
    rm -rf "${SCRIPT_DIR}/binding_glue/autogenerated/tmp"
fi

if [ ! -d "${SCRIPT_DIR}/binding_glue/autogenerated/tmp" ]
then
    mkdir "${SCRIPT_DIR}/binding_glue/autogenerated/tmp" && echo "Created tmp folder"
fi

# TODO: Make sure that the version of the Witness SDK used for bindings is the same as in this package's Cargo.toml.
# TODO: Make sure that the version of Rebase used for bindings is the same as the Witness SDK's Cargo.toml.
# cd "$SCRIPT_DIR"
# TODO: Uncomment above and read Cargo.tomls here!

# Check for deps.
if ! command -v cargo &> /dev/null 
then 
    echo "cargo must be installed and on PATH to run this script, but was not found"
    exit
fi

if ! command -v tsc &> /dev/null 
then 
    echo "tsc must be installed and on PATH to run this script, but was not found"
    exit
fi

if ! command -v wasm-pack &> /dev/null 
then 
    echo "wasm-pack must be installed and on PATH to run this script, but was not found"
    exit
fi

# Generate new rebase bindings
cd "$REBASE_PATH"
if [ -d "${REBASE_PATH}/bindings" ]
then
    rm -rf bindings && echo "Removing old bindings"
fi
cargo test && echo "New Rebase bindings generated"

# Sanity check
if [ ! -d "${REBASE_PATH}/bindings" ]
then
    echo "No bindings were generated from rebase, something went wrong. Try running cargo test at ${REBASE_PATH} to manually repeat the step that failed."
    exit
fi

# Generate new rebase_witness_sdk bindings
cd "${REBASE_WITNESS_PATH}"
if [ -d "${REBASE_WITNESS_PATH}/bindings" ]
then
    rm -rf bindings && echo "Removing old bindings"
fi
cargo test && echo "New Rebase Witness bindings generated"

# Sanity check
if [ ! -d "${REBASE_WITNESS_PATH}/bindings" ]
then
    echo "No bindings were generated from rebase witness sdk, something went wrong. Try running cargo test at ${REBASE_WITNESS_PATH} to manually repeat the step that failed."
    exit
fi

# Build rebase-client
cd "$SCRIPT_DIR"
rm -rf target pkg

# TODO: Most steps from here on need to be duplicated for a "fat" build with node support.
wasm-pack build --scope "$SCOPE" && echo "WASM build complete"

# Sanity check
if [ ! -d "${SCRIPT_DIR}/pkg" ]
then
    echo "No pkg directory was generated from this repo, something went wrong. Try running wasm-pack build at ${SCRIPT_DIR} to manually repeat the step that failed."
    exit
fi

# Copy bindings
mkdir "${SCRIPT_DIR}/pkg/bindings"
cp -a "${REBASE_PATH}/bindings/." "${SCRIPT_DIR}/pkg/bindings/"
cp -a "${REBASE_WITNESS_PATH}/bindings/." "${SCRIPT_DIR}/pkg/bindings/"

# TODO: Implement the rest!

# Generate pkg/bindings/index.ts from list of files.
for binding_file in "$SCRIPT_DIR"/pkg/bindings/*
do
    x=${binding_file%.ts}
    y=${x##*/}
    echo "export {${y}} from './${y}'" >> "$SCRIPT_DIR"/pkg/bindings/index.ts
done

# Add "export * from './bindings/index.ts';" to rebase_client.d.ts
echo "export * from './bindings/index';" >> "$SCRIPT_DIR"/pkg/rebase_client.d.ts

# Generate wrapper types and logic from running tsc on index.ts.
cp -R "$SCRIPT_DIR"/pkg "$SCRIPT_DIR"/binding_glue/autogenerated/tmp
cp "$SCRIPT_DIR"/binding_glue/manual/index.ts "$SCRIPT_DIR"/binding_glue/autogenerated/tmp/pkg

# Add the contents of index.js to the bottom of rebase_client.js


# Add the contents of index.d.ts to rebase_client.d.ts 

# Change pkg/package.json::module to "index.js"

# Sanity check
# TODO: Run a linter to make sure the types line up.
# TODO: Run some sort of dead simple headless browser open-the-lib + ask-the-witness-for-instructions?

# Clean Up
# rm -rf "${SCRIPT_DIR}/binding_glue/autogenerated/tmp"