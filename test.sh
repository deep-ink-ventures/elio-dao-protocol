#!/bin/sh

printf "> Testing core contract...\n"
cd contracts/core && cargo test; cd ../..

printf "> Testing votes contract...\n"
cd contracts/votes && cargo test; cd ../..

printf "> Testing asset contract...\n"
cd contracts/assets && cargo test; cd ../..

