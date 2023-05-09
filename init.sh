#!/bin/sh

echo "> Compiling asset contract ..."
cd contracts/assets && cargo build --target wasm32-unknown-unknown --release && cd ../..
cp contracts/assets/target/wasm32-unknown-unknown/release/elio_assets.wasm wasm

echo "> Compiling votes contract ..."
cd contracts/votes && cargo build --target wasm32-unknown-unknown --release && cd ../..
cp contracts/votes/target/wasm32-unknown-unknown/release/elio_votes.wasm wasm

echo "> Compiling core contract ..."
cd contracts/core && cargo build --target wasm32-unknown-unknown --release && cd ../..
cp contracts/core/target/wasm32-unknown-unknown/release/elio_core.wasm wasm