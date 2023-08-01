# DAO Core Contract
Create a DAO and run.

## Overview
This module contains functionality to create, manage and destroy a DAO alongside with token issuance.
It acts as a central actor of the protocol and provides configuration features and smart contract hook points to fine-tune and customize a DAO with great freedom.

### Interface
- `init(env: Env, votes_id: Address, native_asset_id: Address)`: Initialize the contract.
- `create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao`: Create a DAO.
- `get_dao(env: Env, dao_id: Bytes) -> Dao`: Load a DAO.
- `destroy_dao(env: Env, dao_id: Bytes, dao_owner: Address)`: Destroy a DAO.
- `issue_token(env: Env, dao_id: Bytes, dao_owner: Address, assets_wasm_hash: BytesN<32>, asset_salt: BytesN<32>) -> Address`: Issue a token for a DAO.
- `get_dao_asset_id(env: Env, dao_id: Bytes) -> Address`: Retrieve the DAO asset ID.
- `set_metadata(env: Env, dao_id: Bytes, meta: Bytes, hash: Bytes, dao_owner: Address) -> Metadata`: Set metadata for a DAO.
- `get_metadata(env: Env, dao_id: Bytes) -> Metadata`: Load metadata for a DAO.
- `has_hookpoint(env: Env, dao_id: Bytes) -> bool`: Check if a DAO has a registered hookpoint.
- `get_hookpoint(env: Env, dao_id: Bytes) -> Address`: Retrieve the hookpoint for a DAO.
- `set_hookpoint(env: Env, dao_id: Bytes, hookpoint: Address, dao_owner: Address)`: Set the hookpoint for a DAO.
- `remove_hookpoint(env: Env, dao_id: Bytes, dao_owner: Address)`: Remove the hookpoint for a DAO.
- `change_owner(env: Env, dao_id: Bytes, new_owner: Address, dao_owner: Address) -> Dao`: Transfer ownership of a DAO.
