#![no_std]

use soroban_sdk::{contractimpl, log, Address, Bytes, Env, Symbol, Vec};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

#[cfg(test)]
mod test;

mod types;

mod interface;

use interface::VotesTrait;
use types::{ActiveProposal, Metadata, Proposal, ProposalId};

pub struct VotesContract;

const VOTES: Symbol = Symbol::short("VOTES");
const CORE: Symbol = Symbol::short("CORE");

#[contractimpl]
impl VotesTrait for VotesContract {
    fn init(env: Env, core_id: Address) {
        if env.storage().has(&CORE) {
            panic!("Already initialized")
        }
        env.storage().set(&CORE, &core_id);
    }

    fn get_core_id(env: Env) -> Address {
        env.storage().get_unchecked(&CORE).unwrap()
    }

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

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        Proposal::get_active(&env, dao_id)
    }

    fn get_archived_proposal(env: Env, proposal_id: ProposalId) -> Proposal {
        Proposal::get_archived(&env, proposal_id)
    }

    fn vote(env: Env, dao_id: Bytes, proposal_id: ProposalId, in_favor: bool, voter: Address) {
        voter.require_auth();

        let core_id = Self::get_core_id(env.clone());
        let core = core_contract::Client::new(&env, &core_id);

        log!(&env, "getting DAO token");
        let asset_id = core.get_dao_asset_id(&dao_id);

        Proposal::vote(&env, dao_id, proposal_id, in_favor, voter, asset_id);
    }

    fn fault_proposal(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        reason: Bytes,
        dao_owner: Address,
    ) {
        let core_id = Self::get_core_id(env.clone());
        verify_dao_owner(&env, &dao_id, dao_owner, core_id);

        Proposal::set_faulty(&env, dao_id, proposal_id, reason)
    }

    fn finalize_proposal(env: Env, dao_id: Bytes, proposal_id: ProposalId) {
        Proposal::finalize(&env, dao_id, proposal_id);
    }

    fn mark_implemented(env: Env, proposal_id: ProposalId, dao_owner: Address) {
        let proposal = Proposal::get_archived(&env, proposal_id);

        let core_id = Self::get_core_id(env.clone());
        verify_dao_owner(&env, &proposal.dao_id, dao_owner, core_id);

        Proposal::mark_implemented(&env, proposal_id);
    }
}

fn verify_dao_owner(env: &Env, dao_id: &Bytes, dao_owner: Address, core_id: Address) {
    dao_owner.require_auth();

    let core = core_contract::Client::new(env, &core_id);

    log!(env, "getting DAO");
    let dao = core.get_dao(dao_id);

    log!(env, "verifying DAO owner");
    if dao_owner != dao.owner {
        panic!("not the DAO owner");
    }
}
