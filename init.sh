#!/bin/sh

DIR="$(dirname "$0")"

mkdir -p "${DIR}"/wasm/

for CRATE in core votes assets; do
	printf "> Compiling ${CRATE} contract...\n"
	cargo build -p elio-${CRATE} --target wasm32-unknown-unknown --release &&
		cp "${DIR}"/target/wasm32-unknown-unknown/release/elio_${CRATE}.wasm "${DIR}"/wasm/
done
