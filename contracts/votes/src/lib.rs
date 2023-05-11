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
        // todo: implement

        // this line is to wire the protocol for the assets contract to work
        // as part of the first deliverable - the rest of this functionality is still tbd
        // todo: separate proposals per dao
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

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        // todo: separate proposals per dao
        ActiveProposal::get_all(&env)
    }
}