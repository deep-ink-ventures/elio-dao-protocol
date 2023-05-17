#!/bin/sh

printf "> Compiling asset contract...\n"
cd contracts/assets && cargo build --target wasm32-unknown-unknown --release; cd ../..
cp contracts/assets/target/wasm32-unknown-unknown/release/elio_assets.wasm wasm

printf "> Compiling votes contract...\n"
cd contracts/votes && cargo build --target wasm32-unknown-unknown --release; cd ../..
cp contracts/votes/target/wasm32-unknown-unknown/release/elio_votes.wasm wasm

printf "> Compiling core contract...\n"
cd contracts/core && cargo build --target wasm32-unknown-unknown --release; cd ../..
cp contracts/core/target/wasm32-unknown-unknown/release/elio_core.wasm wasm
