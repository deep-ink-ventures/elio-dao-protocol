use soroban_sdk::{Address, Bytes, Env, Vec};

use crate::types::{ActiveProposal, Configuration, Metadata, Proposal, Voting};

pub trait VotesTrait {
    fn init(env: Env, core_id: Address);

    fn get_core_id(env: Env) -> Address;

    fn create_proposal(env: Env, dao_id: Bytes, proposal_owner: Address) -> u32;

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
        proposal_token_deposit: u128,
        min_threshold_configuration: i128,
        voting: Voting,
        dao_owner: Address,
    );

    fn get_configuration(env: Env, dao_id: Bytes) -> Configuration;

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
