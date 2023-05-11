#!/bin/sh

source .env

printf "\nDeploying votes ...\n"

soroban contract deploy \
    --wasm wasm/elio_votes.wasm \
    --source ${SECRET_KEY} \
    --rpc-url ${RPC_URL} \
    --network-passphrase "${NETWORK_PASSPHRASE}"


printf "\nDeploying core ...\n"

soroban contract deploy \
    --wasm wasm/elio_core.wasm \
    --source ${SECRET_KEY} \
    --rpc-url ${RPC_URL} \
    --network-passphrase "${NETWORK_PASSPHRASE}"


printf "\nDeploying assets ...\n"

soroban contract deploy \
    --wasm wasm/elio_assets.wasm \
    --source ${SECRET_KEY} \
    --rpc-url ${RPC_URL} \
    --network-passphrase "${NETWORK_PASSPHRASE}"