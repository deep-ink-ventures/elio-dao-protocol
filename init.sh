#!/bin/sh

PROFILE="release"
#PROFILE="release-with-logs"

printf "> Compiling core contract (${PROFILE})...\n"
pushd contracts/core
cargo build --target wasm32-unknown-unknown --profile ${PROFILE} &&
	cp target/wasm32-unknown-unknown/${PROFILE}/elio_core.wasm ../../wasm
popd > /dev/zero

printf "> Compiling votes contract (${PROFILE})...\n"
pushd contracts/votes
cargo build --target wasm32-unknown-unknown --profile ${PROFILE} &&
	cp target/wasm32-unknown-unknown/${PROFILE}/elio_votes.wasm ../../wasm
popd > /dev/zero

printf "> Compiling asset contract (${PROFILE})...\n"
pushd contracts/assets
cargo build --target wasm32-unknown-unknown --profile ${PROFILE} &&
	cp target/wasm32-unknown-unknown/${PROFILE}/elio_assets.wasm ../../wasm
popd > /dev/zero
