use soroban_sdk::{Env, Bytes, Address, BytesN};

use crate::types::{Dao, Metadata};

pub trait CoreTrait {
    /// Initialize the contract
    ///
    /// - `votes_wasm_hash`: The wasm hash of the votes contract
    /// - `votes_salt`: a 32 bytes salt to derive the contract id
    ///
    fn init(env: Env, votes_id: Address, native_asset_id: Address);

    /// Gets the Votes ID
    fn get_votes_id(env: Env) -> Address;

    /// Gets the Native Asset ID
    fn get_native_asset_id(env: Env) -> Address;

    /// Create a fresh DAO.
    ///
    /// - `dao_id`: Unique identifier for the DAO
    /// - `dao_name`: Name of the to-be-created DAO
    /// - `dao_owner`: The owner of the freshly created dao
    ///
    fn create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao;

    /// Load a DAO.
    ///
    /// - `dao_id`: The id of the dao to load;
    fn get_dao(env: Env, dao_id: Bytes) -> Dao;

    /// Destroy a DAO.
    ///
    /// - `dao_id`: The DAO to destroy
    /// - `dao_owner`: The owner of to-be-destroyed dao
    fn destroy_dao(env: Env, dao_id: Bytes, dao_owner: Address);

    /// Issue the DAO token
    ///
    /// - `dao_id`: The DAO for which to issue a token
    /// - `dao_owner`: The owner of the dao about to issue a token
    /// - `assets_wasm_hash`: The wasm hash of the assets contract
    /// - `asset_salt`: a 32 bytes salt to derive the contract id
    ///
    /// Tokens can only be issued once and the signer of this TX needs to be the owner
    /// of the DAO.
    fn issue_token(env: Env, dao_id: Bytes, dao_owner: Address, assets_wasm_hash: BytesN<32>, asset_salt: BytesN<32>) -> Address;

    /// Returns the contract id of the dao asset (if exists).
    ///
    /// - `dao_id`: The id of the dao to load;
    ///
    fn get_dao_asset_id(env: Env, dao_id: Bytes) -> Address;

    /// Set metadata
    ///
    /// - `dao_id`: The DAO for which to set metadata
    /// - `meta`: HTTP or IPFS address for the metadata about this DAO (description, logo)
    /// - `hash`: SHA3 hash of the metadata to be found via `meta`
    /// - `dao_owner`: the current owner of the dao
    fn set_metadata(env: Env, dao_id: Bytes, meta: Bytes, hash: Bytes, dao_owner: Address) -> Metadata;

    /// Load metadata for a dao
    ///
    /// - `dao_id`: The id of the dao to load meta data for;
    fn get_metadata(env: Env, dao_id: Bytes) -> Metadata;

    /// Checks if the dao has a registered hookpoint
    ///
    /// - `dao_id`: The DAO for which to set the hookpoint
    fn has_hookpoint(env: Env, dao_id: Bytes) -> bool;

    /// Get the hookpoint if configured for this contract
    ///
    /// - `dao_id`: The DAO for which to set the hookpoint
    fn get_hookpoint(env: Env, dao_id: Bytes) -> Address;

    /// Set the hookpoint for this contract
    ///
    /// - `dao_id`: The DAO for which to set the hookpoint
    /// - `hookpoint`: The address of the deployed hookpoint contract
    /// - `dao_owner`: the current owner of the dao
    fn set_hookpoint(env: Env, dao_id: Bytes, hookpoint: Address, dao_owner: Address);

    /// Remove the hookpoint
    ///
    /// - `dao_id`: The DAO for which to set the hookpoint
    /// - `dao_owner`: the current owner of the dao
    fn remove_hookpoint(env: Env, dao_id: Bytes, dao_owner: Address);

    /// Change owner
    ///
    /// - `dao_id`: the DAO to transfer ownership of
    /// - `new_owner`: the new owner
    /// - `dao_owner`: the current owner of the dao
    fn change_owner(env: Env, dao_id: Bytes, new_owner: Address, dao_owner: Address) -> Dao;
}
