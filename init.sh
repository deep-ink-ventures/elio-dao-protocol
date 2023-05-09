#!/bin/sh

echo "> Compiling asset contracts ..."
cd contracts/assets && cargo build --target wasm32-unknown-unknown --release && cd ../..
cp contracts/assets/target/wasm32-unknown-unknown/release/elio_assets.wasm wasm

echo "> Compiling votes contracts ..."
#cd contracts/votes && cargo build --target wasm32-unknown-unknown --release && cd ../..
#cp contracts/votes/target/wasm32-unknown-unknown/release/elio_votes.wasm wasm
