#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, Env, Symbol, Vec};

#[cfg(test)]
mod test;

mod types;

mod interface;

use interface::VotesTrait;
use types::{ActiveProposal, Metadata, Proposal, ProposalId};

pub struct VotesContract;

const VOTES: Symbol = Symbol::short("VOTES");

#[contractimpl]
impl VotesTrait for VotesContract {
    fn create_proposal(env: Env, dao_id: Bytes, owner: Address) -> ProposalId {
        Proposal::create(&env, dao_id, owner)
    }

    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        meta: Bytes,
        hash: Bytes,
        proposal_owner: Address,
    ) {
        Metadata::set(
            &env,
            dao_id,
            proposal_id,
            meta.clone(),
            hash.clone(),
            proposal_owner,
        );
        env.events()
            .publish((VOTES, Symbol::short("meta_set")), (meta, hash));
    }

    fn get_metadata(env: Env, proposal_id: ProposalId) -> Metadata {
        Metadata::get(&env, proposal_id)
    }

    fn fault_proposal(env: Env, proposal_id: ProposalId, reason: Bytes, dao_owner: Address) {
        // todo: implement
    }

    fn finalize_proposal(env: Env, proposal_id: ProposalId) {
        // todo: implement
    }

    fn vote(env: Env, dao_id: Bytes, proposal_id: ProposalId, in_favor: bool, voter: Address) {
        ActiveProposal::vote(env, dao_id, proposal_id, in_favor, voter);
    }

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        Proposal::get_active(&env, dao_id)
    }
}
