use soroban_sdk::{Address, Bytes, Env, Vec};

use crate::types::{ActiveProposal, Configuration, Metadata, Proposal, ProposalId, Voting};

pub trait VotesTrait {
    fn init(env: Env, core_id: Address);

    fn get_core_id(env: Env) -> Address;

    fn create_proposal(env: Env, dao_id: Bytes, proposal_owner: Address) -> ProposalId;

    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        meta: Bytes,
        hash: Bytes,
        proposal_owner: Address,
    );

    fn get_metadata(env: Env, proposal_id: ProposalId) -> Metadata;

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal>;

    fn get_archived_proposal(env: Env, id: ProposalId) -> Proposal;

    fn set_configuration(
        env: Env,
        dao_id: Bytes,
        proposal_duration: u32,
        proposal_token_deposit: u128,
        voting: Voting,
        dao_owner: Address,
    );

    fn get_configuration(env: Env) -> Configuration;

    fn vote(env: Env, dao_id: Bytes, proposal_id: ProposalId, in_favor: bool, voter: Address);

    fn fault_proposal(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        reason: Bytes,
        dao_owner: Address,
    );

    fn finalize_proposal(env: Env, dao_id: Bytes, proposal_id: ProposalId);

    fn mark_implemented(env: Env, proposal_id: ProposalId, dao_owner: Address);
}
