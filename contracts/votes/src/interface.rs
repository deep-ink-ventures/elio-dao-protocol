use soroban_sdk::{Address, Bytes, BytesN, Env, Vec};

use crate::types::{ActiveProposal, Metadata, Proposal, ProposalId};

pub trait VotesTrait {
    fn init(env: Env, core_id: BytesN<32>);

    fn get_core_id(env: Env) -> BytesN<32>;

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
