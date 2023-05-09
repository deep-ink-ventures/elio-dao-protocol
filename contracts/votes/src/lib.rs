#![no_std]

use soroban_sdk::{contractimpl, Env, Bytes, Address, Vec};

#[cfg(test)]
mod test;

mod types;

mod interface;
use interface::VotesTrait;
use types::ActiveProposal;

pub struct VotesContract;


#[contractimpl]
impl VotesTrait for VotesContract {

    fn create_proposal(env: Env, dao_id: Bytes, proposal_id: Bytes) {
        // todo: https://github.com/users/deep-ink-ventures/projects/2?pane=issue&itemId=26775445

        // leave the following intact when implementing the rest.
        ActiveProposal::add(&env, proposal_id)
    }

    fn set_metadata(env: Env, proposal_id: Bytes, meta: Bytes, hash: Bytes, proposal_owner: Address) {
        // todo: implement
    }

    fn fault_proposal(env: Env, proposal_id: Bytes, reason: Bytes, dao_owner: Address) {
        // todo: implement
    }

    fn finalize_proposal(env: Env, proposal_id: Bytes) {
        // todo: implement
    }

    fn vote(env: Env, proposal_id: Bytes, in_favor: bool, voter: Address) {
        // todo: implement
    }

    fn get_active_proposals(env: Env) -> Vec<ActiveProposal> {
        ActiveProposal::get_all(&env)
    }
}