#!/bin/sh

source .env

printf "\nDeploying core ...\n"

soroban contract deploy \
    --wasm wasm/elio_core.wasm \
    --source ${SECRET_KEY} \
    --rpc-url ${RPC_URL} \
    --network-passphrase "${NETWORK_PASSPHRASE}"


printf "\nInstalling assets ...\n"

soroban contract install \
    --wasm wasm/elio_assets.wasm \
    --source ${SECRET_KEY} \
    --rpc-url ${RPC_URL} \
    --network-passphrase "${NETWORK_PASSPHRASE}"

printf "\nInstalling votes ...\n"
soroban contract install \
    --wasm wasm/elio_votes.wasm \
    --source ${SECRET_KEY} \
    --rpc-url ${RPC_URL} \
    --network-passphrase "${NETWORK_PASSPHRASE}"