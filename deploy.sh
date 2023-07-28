#!/bin/sh

source .env

STELLAR_ASSET_ID="$(
  soroban lab token id \
   --source "${SECRET_KEY}" \
   --rpc-url "${RPC_URL}" \
   --network-passphrase "${NETWORK_PASSPHRASE}" \
   --asset native
)"

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

printf "\nInitialising core ...\n"
soroban contract invoke \
    --id "${CORE_ADDRESS}" \
    --source "${SECRET_KEY}" \
    --rpc-url "${RPC_URL}" \
    --network-passphrase "${NETWORK_PASSPHRASE}" \
    -- \
    init \
    --votes_id "${VOTES_ADDRESS}" \
    --native_asset_id "${STELLAR_ASSET_ID}"

for CONTRACT in core votes assets; do
	printf "\nBumping contract ${CONTRACT} ...\n"
	soroban contract bump \
   --source "${SECRET_KEY}" \
   --rpc-url "${RPC_URL}" \
   --network-passphrase "${NETWORK_PASSPHRASE}" \
   --wasm wasm/elio_${CONTRACT}.wasm \
   --durability persistent \
   --ledgers-to-expire 6312000
done

printf "\n Settings instance storage for core ...\n"
soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --id $CORE_ADDRESS \
 --key-xdr AAAAFA== \
 --durability persistent \
 --ledgers-to-expire 6312000

printf "\n Settings instance storage for votes ...\n"
soroban contract bump \
 --source "${SECRET_KEY}" \
 --rpc-url "${RPC_URL}" \
 --network-passphrase "${NETWORK_PASSPHRASE}" \
 --id $VOTES_ADDRESS \
 --key-xdr AAAAFA== \
 --durability persistent \
 --ledgers-to-expire 6312000

#printf "\nUpdate Service"
#curl -XPATCH -H "Config-Secret: ${CONFIG_SECRET}" -H "Content-type: application/json" -d "{
#  \"core_contract_address\": \"${CORE_ADDRESS}\",
#  \"votes_contract_address\": \"${VOTES_ADDRESS}\",
#  \"assets_wasm_hash\": \"${ASSETS_WASM_HASH}\",
#  \"blockchain_url\": \"${RPC_URL}\",
#  \"network_passphrase\": \"${NETWORK_PASSPHRASE}\"
#}" "${SERVICE_URL}/update-config/"

printf "\nRPC_URL=$RPC_URL"
printf "\nCORE_ADDRESS=$CORE_ADDRESS"
printf "\nVOTES_ADDRESS=$VOTES_ADDRESS"
printf "\nASSETS_WASM_HASH=$ASSETS_WASM_HASH\n"
