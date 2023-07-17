#!/bin/sh

source .env

printf "\nDeploying core ...\n"
CORE_ADDRESS="$(
soroban contract deploy \
    --wasm wasm/elio_core.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export CORE_ADDRESS

printf "\nDeploying votes ...\n"
VOTES_ADDRESS="$(
soroban contract deploy \
    --wasm wasm/elio_votes.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export VOTES_ADDRESS

printf "\nInstalling assets ...\n"
ASSETS_WASM_HASH="$(
soroban contract install \
    --wasm wasm/elio_assets.wasm \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}"
)"
export ASSETS_WASM_HASH

printf "\n Initialising core ...\n"
soroban contract invoke \
    --id "${CORE_ADDRESS}" \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}" \
    -- \
    init \
    --votes_id "${VOTES_ADDRESS}" \
    --native_asset_id "${STELLAR_ASSET_ID}"

printf "\n Update Service"
curl -XPATCH -H "Config-Secret: ${CONFIG_SECRET}" -H "Content-type: application/json" -d "{
  "core_contract_address": "${CORE_ADDRESS}",
  "votes_contract_address": "${VOTES_ADDRESS}",
  "assets_wasm_hash": "${ASSETS_WASM_HASH}"
}" "${SERVICE_URL}/update-config/'

printf "\nCORE ADDRESS: $CORE_ADDRESS"
printf "\nVOTES ADDRESS: $VOTES_ADDRESS"
printf "\nASSETS WASM HASH: $ASSETS_WASM_HASH\n"
