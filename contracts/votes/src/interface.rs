use soroban_sdk::{Address, Bytes, Env, Vec};

use crate::types::{ActiveProposal, Configuration, Metadata, Proposal};

pub trait VotesTrait {
    /// Initialize the contract
    ///
    /// - `core_id`: The address of the core-contract.
    fn init(env: Env, core_id: Address);

    /// Gets the core_id
    fn get_core_id(env: Env) -> Address;

    /// Create a proposal
    ///
    /// - `dao_id`: The id of the DAO where proposal is created.
    /// - `proposal_owner`: The address of owner of the proposal.
    fn create_proposal(env: Env, dao_id: Bytes, proposal_owner: Address) -> u32;

    /// Sets the metadata
    ///
    /// - `dao_id`: The dao id where metadata has been set
    /// - `proposal_id`: The id of the proposal
    /// - `meta`: The meta combined with hash to produce metadata
    /// - `hash`: The hash combined with meta to produce metadata
    /// - `proposal_owner`: The owner of the proposal
    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        proposal_id: u32,
        meta: Bytes,
        hash: Bytes,
        proposal_owner: Address,
    );

    /// Gets the metadata of a proposal
    ///
    /// - `proposal_id`: The id of the proposal
    fn get_metadata(env: Env, proposal_id: u32) -> Metadata;

    /// Gets all proposals that are active
    ///
    /// - `dao_id`: The id of the DAO where proposal are stored.
    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal>;

    /// Gets a proposal that is inactive
    ///
    /// - `id`: The id of the proposal we are trying to fetch.
    fn get_archived_proposal(env: Env, id: u32) -> Proposal;

    /// Set the configuration of the dao
    ///
    /// - `dao_id`: The id of the DAO we are trying to configure.
    /// - `proposal_duration`: The amount of blocks the proposal is active.
    /// - `min_threshold_configuration`: The min voting power required.
    /// - `dao_owner`: The owner of the DAO. Required for validation.
    fn set_configuration(
        env: Env,
        dao_id: Bytes,
        proposal_duration: u32,
        min_threshold_configuration: i128,
        dao_owner: Address,
    ) -> Configuration;

    /// Gets the configuration of the dao
    ///
    /// - `dao_id`: The id of the DAO we are trying to get the configuration.
    fn get_configuration(env: Env, dao_id: Bytes) -> Configuration;

    /// Checks if the dao has a configuration
    ///
    /// - `dao_id`: The id of the DAO we are trying to check the configuration.
    fn has_configuration(env: Env, dao_id: Bytes) -> bool;

    /// Removes the configuration of the dao
    ///
    /// - `dao_id`: The id of the DAO we are trying to remove the configuration.
    fn remove_configuration(env: Env, dao_id: Bytes, dao_owner: Address);

    /// User casts his/her vote in for the proposal
    ///
    /// - `dao_id`: The id of the DAO we are trying vote.
    /// - `proposal_id`: The id of the proposal we are trying to vote.
    /// - `in_favor`: Boolean whether voter is in favor or not.
    /// - `voter`: Address of the voter.
    fn vote(env: Env, dao_id: Bytes, proposal_id: u32, in_favor: bool, voter: Address) -> i128 ;

    /// Declare a proposal as faulty
    ///
    /// - `dao_id`: The dao id that will be declared faulty.
    /// - `proposal_id`: The id of the proposal to be declared faulty.
    /// - `reason`: The reason of declaring the proposal faulty.
    /// - `dao_owner`: The owner of the DAO. Required for validation.
    fn fault_proposal(
        env: Env,
        dao_id: Bytes,
        proposal_id: u32,
        reason: Bytes,
        dao_owner: Address,
    );

    /// Declare a proposal as finalized
    ///
    /// - `dao_id`: The dao id that will be finalized for.
    /// - `proposal_id`: The id of the proposal that will be declared finalized.
    fn finalize_proposal(env: Env, dao_id: Bytes, proposal_id: u32);

    /// Declare a proposal as implemented
    ///
    /// - `proposal_id`: The id of the proposal that will be declared marked implemented.
    /// - `dao_owner`: The owner of the DAO. Required for validation.
    fn mark_implemented(env: Env, proposal_id: u32, dao_owner: Address);
}
