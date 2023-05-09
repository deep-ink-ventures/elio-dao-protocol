#![no_std]

use soroban_sdk::{contractimpl, Env, Bytes, Address, Symbol};

#[cfg(test)]
mod test;

mod interface;
use interface::VotesTrait;

pub struct VotesContract;


#[contractimpl]
impl VotesTrait for VotesContract {

    fn create_proposal(env: Env, dao_id: Symbol, proposal_id: Symbol, proposal_owner: Address) {
        // todo: implement
    }

    fn set_metadata(env: Env, proposal_id: Symbol, meta: Bytes, hash: Bytes, proposal_owner: Address) {
        // todo: implement
    }

    fn fault_proposal(env: Env, proposal_id: Symbol, reason: Bytes, dao_owner: Address) {
        // todo: implement
    }

    fn finalize_proposal(env: Env, proposal_id: Symbol) {
        // todo: implement
    }

    fn vote(env: Env, proposal_id: Symbol, in_favor: bool, voter: Address) {
        // todo: implement
    }
    
}