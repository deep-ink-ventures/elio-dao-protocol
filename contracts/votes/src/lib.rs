#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, Env, Symbol, Vec};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

#[cfg(test)]
mod test;

mod types;

mod interface;

use core_contract::Client as CoreContractClient;
use interface::VotesTrait;
use types::{ActiveProposal, Metadata, Proposal, ProposalId};
use crate::types::{Configuration, Voting};

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

    fn create_proposal(env: Env, dao_id: Bytes, proposal_owner: Address) -> ProposalId {
        let core_id = Self::get_core_id(env.clone());
        let core = CoreContractClient::new(&env, &core_id);

        // check that DAO exists
        let _ = core.get_dao(&dao_id);

        Proposal::create(&env, dao_id, proposal_owner)
    }

    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        meta: Bytes,
        hash: Bytes,
        proposal_owner: Address,
    ) {
        // todo: this should only be set once
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

    fn set_configuration(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        proposal_duration: u32,
        proposal_token_deposit: u128,
        voting: Voting,
        proposal_owner: Address,
    ) {
        Configuration::set(
            &env,
            dao_id,
            proposal_id,
            proposal_duration,
            proposal_token_deposit,
            voting.clone(),
            proposal_owner
        );
        env.events()
            .publish(
                (VOTES, Symbol::short("conf_set")),
                (proposal_duration, proposal_token_deposit, voting)
            );
    }

    fn get_configuration(env: Env, proposal_id: ProposalId) -> Configuration {
        Configuration::get(&env, proposal_id)
    }

    fn vote(env: Env, dao_id: Bytes, proposal_id: ProposalId, in_favor: bool, voter: Address) {
        voter.require_auth();

        let core_id = Self::get_core_id(env.clone());
        let core = core_contract::Client::new(&env, &core_id);

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

    let dao = core.get_dao(dao_id);

    if dao_owner != dao.owner {
        panic!("not the DAO owner");
    }
}
