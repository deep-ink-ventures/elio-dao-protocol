use soroban_sdk::{Env, Symbol, Bytes, Address};

use crate::types::Dao;

pub trait CoreTrait {
    
    /// Create a fresh DAO.
	///
	/// - `dao_id`: Unique identifier for the DAO, bounded by _MinLength_ & _MaxLengthId_
	/// - `dao_name`: Name of the to-be-created DAO, bounded by _MinLength_ & _MaxLengthName_
	/// - `dao_owner`: The owner of the freshly created dao
    ///
    fn create_dao(env: Env, dao_id: Symbol, dao_name: Bytes, dao_owner: Address) -> Dao;
    
    /// Load a DAO.
    ///
    /// - `dao_id`: The id of the dao to load;
    fn get_dao(env: Env, dao_id: Symbol) -> Dao;
    
    /// Destroy a DAO.
    ///
	/// - `dao_id`: The DAO to destroy
    /// - `dao_owner`: The owner of to-be-destroyed dao
    fn destroy_dao(env: Env, dao_id: Symbol, dao_owner: Address);
    
    /// Issue the DAO token
    ///
    /// - `dao_id`: The DAO for which to issue a token
    /// - `supply`: The total supply of the token to be issued
    /// - `dao_owner`: The owner of the dao about to issue a token
    ///
    /// Tokens can only be issued once and the signer of this TX needs to be the owner
    /// of the DAO.
    fn issue_token(env: Env, dao_id: Symbol, supply: i128, dao_owner: Address);
    
    /// Set metadata
    ///
	/// - `dao_id`: The DAO for which to set metadata
	/// - `meta`: HTTP or IPFS address for the metadata about this DAO (description, logo)
	/// - `hash`: SHA3 hash of the metadata to be found via `meta`
    /// - `dao_owner`: the current owner of the dao
    fn set_metadata(env: Env, dao_id: Symbol, meta: Bytes, hash: Bytes, dao_owner: Address);
    
    /// Change owner
	///
	/// - `dao_id`: the DAO to transfer ownership of
	/// - `new_owner`: the new owner
    /// - `dao_owner`: the current owner of the dao
    fn change_owner(env: Env, dao_id: Symbol, new_owner: Address, dao_owner: Address) -> Dao;
}