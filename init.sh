#!/bin/sh

DIR="$(dirname "$0")"

PROFILE="release";
#PROFILE="release-with-logs";

mkdir -p "${DIR}"/wasm/

for CRATE in core votes assets; do
	printf "> Compiling ${CRATE} contract...\n"
	cargo build -p elio-${CRATE} --target wasm32-unknown-unknown --profile "${PROFILE}" &&
		cp "${DIR}"/target/wasm32-unknown-unknown/"${PROFILE}"/elio_${CRATE}.wasm "${DIR}"/wasm/
done
