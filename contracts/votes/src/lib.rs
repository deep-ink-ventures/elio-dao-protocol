#![no_std]

use soroban_sdk::{contractimpl, contract, Address, Bytes, Env, Symbol, Vec, panic_with_error, symbol_short};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

#[cfg(test)]
mod test;

mod types;

mod interface;

mod hooks;
mod events;
mod error;

use core_contract::Client as CoreContractClient;
use events::{
    ProposalFaultedEventData, ProposalMetadataSetEventData, CORE,
    CREATED, FAULTED, METADATA_SET, PROPOSAL, CONF_SET, ProposalConfigurationSetEventData,
};
use interface::VotesTrait;
use types::{ActiveProposal, Metadata, Proposal};
use crate::error::VotesError;
use crate::types::{Configuration};

use crate::events::{ProposalCreatedEventData, VoteCastEventData, VOTE_CAST};
use crate::hooks::on_before_mark_implemented;

#[contract]
pub struct VotesContract;

pub const NATIVE: Symbol = symbol_short!("NATIVE");

#[contractimpl]
impl VotesTrait for VotesContract {
    fn init(env: Env, core_id: Address) {
        if env.storage().instance().has(&CORE) {
            panic_with_error!(env, VotesError::CoreAlreadyInitialized)
        }
        env.storage().instance().set(&CORE, &core_id);
    }

    fn get_core_id(env: Env) -> Address {
        env.storage().instance().get(&CORE).unwrap()
    }

    fn create_proposal(env: Env, dao_id: Bytes, proposal_owner: Address) -> u32 {
        let core_id = Self::get_core_id(env.clone());
        let core = CoreContractClient::new(&env, &core_id);

        // check that DAO exists
        let _ = core.get_dao(&dao_id);
        // check that configuration exists
        Self::get_configuration(env.clone(), dao_id.clone());
        let proposal_id = Proposal::create(&env, dao_id.clone(), proposal_owner.clone(), core_id);
        env.events().publish(
            (PROPOSAL, CREATED),
            ProposalCreatedEventData {
                proposal_id,
                dao_id,
                owner_id: proposal_owner,
            },
        );
        proposal_id
    }

    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        proposal_id: u32,
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
        env.events().publish(
            (PROPOSAL, METADATA_SET),
            ProposalMetadataSetEventData {
                proposal_id,
                url: meta,
                hash,
            },
        );
    }

    fn get_metadata(env: Env, proposal_id: u32) -> Metadata {
        Metadata::get(&env, proposal_id)
    }

    fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        Proposal::get_active(&env, dao_id)
    }

    fn get_archived_proposal(env: Env, proposal_id: u32) -> Proposal {
        Proposal::get_archived(&env, proposal_id)
    }

    fn set_configuration(
        env: Env,
        dao_id: Bytes,
        proposal_duration: u32,
        min_threshold_configuration: i128,
        dao_owner: Address,
    ) -> Configuration {
        verify_dao_owner(&env, &dao_id, dao_owner, Self::get_core_id(env.clone()));
        let configuration = Configuration::set(
            &env,
            dao_id.clone(),
            proposal_duration,
            min_threshold_configuration,
        );
        env.events()
            .publish(
                (PROPOSAL, CONF_SET),
                ProposalConfigurationSetEventData {
                    dao_id,
                    proposal_duration,
                    min_threshold_configuration,
                }
            );
        configuration
    }

    fn get_configuration(env: Env, dao_id: Bytes) -> Configuration {
        Configuration::get(&env, dao_id)
    }

    fn has_configuration(env: Env, dao_id: Bytes) -> bool {
        env.storage().persistent().has(&dao_id)
    }

    fn remove_configuration(env: Env, dao_id: Bytes, dao_owner: Address) {
        let core_id = Self::get_core_id(env.clone());

        verify_dao_owner(&env, &dao_id, dao_owner, core_id);
        Configuration::remove(&env, dao_id)
    }

    fn vote(env: Env, dao_id: Bytes, proposal_id: u32, in_favor: bool, voter: Address) -> i128 {
        voter.require_auth();

        let core_id = Self::get_core_id(env.clone());
        let core = core_contract::Client::new(&env, &core_id);

        let asset_id = core.get_dao_asset_id(&dao_id);

        let voting_power = Proposal::vote(&env, dao_id, proposal_id, in_favor, voter.clone(), asset_id);
        env.events().publish(
            (PROPOSAL, VOTE_CAST),
            VoteCastEventData {
                proposal_id,
                voter_id: voter,
                in_favor,
                voting_power
            },
        );
        voting_power
    }

    fn fault_proposal(
        env: Env,
        dao_id: Bytes,
        proposal_id: u32,
        reason: Bytes,
        dao_owner: Address,
    ) {
        let core_id = Self::get_core_id(env.clone());
        verify_dao_owner(&env, &dao_id, dao_owner, core_id);

        Proposal::set_faulty(&env, dao_id, proposal_id, reason.clone());
        env.events().publish(
            (PROPOSAL, FAULTED),
            ProposalFaultedEventData {
                proposal_id,
                reason,
            },
        );
    }

    fn finalize_proposal(env: Env, dao_id: Bytes, proposal_id: u32) {
        Proposal::finalize(&env, dao_id, proposal_id);
    }

    fn mark_implemented(env: Env, proposal_id: u32, dao_owner: Address) {
        let proposal = Proposal::get_archived(&env, proposal_id);

        let core_id = Self::get_core_id(env.clone());
        verify_dao_owner(&env, &proposal.dao_id, dao_owner, core_id);
        on_before_mark_implemented(&env, &proposal.dao_id, proposal_id);

        Proposal::mark_implemented(&env, proposal_id);
    }
}

fn verify_dao_owner(env: &Env, dao_id: &Bytes, dao_owner: Address, core_id: Address) {
    dao_owner.require_auth();

    let core = core_contract::Client::new(env, &core_id);

    let dao = core.get_dao(dao_id);

    if dao_owner != dao.owner {
        panic_with_error!(env, VotesError::NotDaoOwner)
    }
}