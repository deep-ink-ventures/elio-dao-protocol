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

    /// Create a proposal
    ///
    /// - `dao_id`: The id of the DAO where proposal is created.
    /// - `proposal_owner`: The address of owner of the proposal.
    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        proposal_id: u32,
        meta: Bytes,
        hash: Bytes,
        proposal_owner: Address,
    );

    fn get_metadata(env: Env, proposal_id: u32) -> Metadata;

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal>;

    fn get_archived_proposal(env: Env, id: u32) -> Proposal;

    fn set_configuration(
        env: Env,
        dao_id: Bytes,
        proposal_duration: u32,
        min_threshold_configuration: i128,
        dao_owner: Address,
    ) -> Configuration;

    fn get_configuration(env: Env, dao_id: Bytes) -> Configuration;

    fn has_configuration(env: Env, dao_id: Bytes) -> bool;

    fn remove_configuration(env: Env, dao_id: Bytes, dao_owner: Address);

    fn vote(env: Env, dao_id: Bytes, proposal_id: u32, in_favor: bool, voter: Address) -> i128 ;

    fn fault_proposal(
        env: Env,
        dao_id: Bytes,
        proposal_id: u32,
        reason: Bytes,
        dao_owner: Address,
    );

    fn finalize_proposal(env: Env, dao_id: Bytes, proposal_id: u32);

    fn mark_implemented(env: Env, proposal_id: u32, dao_owner: Address);
}
