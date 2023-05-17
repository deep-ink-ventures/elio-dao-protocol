#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, Env, Vec};

#[cfg(test)]
mod test;

mod types;

mod interface;
use interface::VotesTrait;
use types::{ActiveProposal, Proposal, ProposalId};

pub struct VotesContract;

#[contractimpl]
impl VotesTrait for VotesContract {
    fn create_proposal(env: Env, dao_id: Bytes, owner: Address) -> ProposalId {
        Proposal::create(&env, dao_id, owner)
    }

    fn set_metadata(
        env: Env,
        proposal_id: Bytes,
        meta: Bytes,
        hash: Bytes,
        proposal_owner: Address,
    ) {
        todo!();
    }

    fn fault_proposal(env: Env, proposal_id: Bytes, reason: Bytes, dao_owner: Address) {
        // todo: implement
    }

    fn finalize_proposal(env: Env, proposal_id: Bytes) {
        // todo: implement
    }

    fn vote(env: Env, proposal_id: Bytes, in_favor: bool, voter: Address) {
        todo!()
    }

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        Proposal::get_active(&env, dao_id)
    }
}
