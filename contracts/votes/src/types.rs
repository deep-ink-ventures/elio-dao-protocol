use soroban_sdk::{contracttype, Address, Bytes, Env, IntoVal, Symbol, Vec, token, panic_with_error};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}
use core_contract::Client as CoreContractClient;

use crate::error::VotesError;


use crate::events::{ProposalStatusUpdateEventData, STATUS_UPDATE, PROPOSAL, CORE};
use crate::hooks::on_vote;

#[contracttype]
struct ActiveKey(Bytes);

#[contracttype]
struct ArchiveKey(u32);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub dao_id: Bytes,
    pub ledger: u32,
    pub owner: Address,
    pub status: PropStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveProposal {
    pub id: u32,
    pub in_favor: i128,
    pub against: i128,
    pub inner: Proposal,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PropStatus {
    Running,
    Accepted,
    Rejected,
    Faulty(Bytes),
    Implemented,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Voting {
    MAJORITY,
    CUSTOM,
}

pub const XLM: i128 = 10_000_000;
pub const RESERVE_AMOUNT: i128 = 100 * XLM;
pub const PROPOSAL_MAX_NR: u32 = 25;

const PROP_ID: Symbol = Symbol::short("PROP_ID");

impl Proposal {
    pub fn create(env: &Env, dao_id: Bytes, owner: Address, core_id: Address) -> u32 {
        owner.require_auth();

        let mut proposals = Self::get_active(env, dao_id.clone());
        if proposals.len() == PROPOSAL_MAX_NR {
            panic_with_error!(env, VotesError::MaxProposalsReached)
        }

        // Transfer required amount to prevent spam
        let core = CoreContractClient::new(&env, &core_id);
        let native_asset_id = core.get_native_asset_id();
        let native_token = token::Client::new(&env, &native_asset_id);
        let contract = env.current_contract_address();
        native_token.transfer(&owner, &contract, &RESERVE_AMOUNT);

        let id = env.storage().get(&PROP_ID).unwrap_or(Ok(0)).unwrap();
        proposals.push_back(ActiveProposal {
            id,
            in_favor: 0,
            against: 0,
            inner: Proposal {
                dao_id: dao_id.clone(),
                ledger: env.ledger().sequence(),
                status: PropStatus::Running,
                owner,
            },
        });
        env.storage().set(&ActiveKey(dao_id), &proposals);
        env.storage().set(&PROP_ID, &(id + 1));
        id
    }

    pub fn get_active(env: &Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        let key = ActiveKey(dao_id.clone());
        if !env.storage().has(&key) {
            return Vec::new(env);
        }
        let active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        let mut filtered_proposals: Vec<ActiveProposal> = Vec::new(env);

        let proposal_duration = Configuration::get(&env, dao_id).proposal_duration;

        // filter out outdated proposals
        let len = active_proposals.len();
        for proposal in active_proposals.into_iter_unchecked() {
            if env.ledger().sequence() <= proposal.inner.ledger + proposal_duration {
                filtered_proposals.push_back(proposal);
            }
        }
        if filtered_proposals.len() < len {
            env.storage().set(&key, &filtered_proposals);
        }

        filtered_proposals
    }

    pub fn get_archived(env: &Env, proposal_id: u32) -> Proposal {
        let key = ArchiveKey(proposal_id);
        env.storage().get_unchecked(&key).unwrap()
    }

    pub fn vote(
        env: &Env,
        dao_id: Bytes,
        proposal_id: u32,
        in_favor: bool,
        voter: Address,
        asset_id: Address,
    ) -> i128 {
        let key = ActiveKey(dao_id.clone());
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id.clone() {
                let voting_power_pre_hook: i128 = env.invoke_contract(
                    &asset_id,
                    &Symbol::new(env, "get_balance_at"),
                    (voter.clone(), p.inner.ledger).into_val(env),
                );
                let voting_power = on_vote(env, &dao_id, &proposal_id, &voter, voting_power_pre_hook);

                if in_favor {
                    p.in_favor += voting_power;
                } else {
                    p.against += voting_power;
                }
                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);
                return voting_power
            }
        }
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn set_faulty(env: &Env, dao_id: Bytes, proposal_id: u32, reason: Bytes) {
        let key = ActiveKey(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                p.inner.status = PropStatus::Faulty(reason);

                // return reserved tokens
                let core_id = env.storage().get_unchecked(&CORE).unwrap();
                let core = CoreContractClient::new(&env, &core_id);
                let native_asset_id = core.get_native_asset_id();
                let native_token = token::Client::new(&env, &native_asset_id);
                let contract = env.current_contract_address();
                native_token.transfer(&contract, &p.inner.owner, &RESERVE_AMOUNT);

                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);
                return;
            }
        }
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn finalize(env: &Env, dao_id: Bytes, proposal_id: u32) {
        let key = ActiveKey(dao_id.clone());
        let configuration = Configuration::get(&env, dao_id);
        let proposal_duration = configuration.proposal_duration;
        let min_threshold_configuration = configuration.min_threshold_configuration;
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                if env.ledger().sequence() <= p.inner.ledger + proposal_duration {
                    panic_with_error!(env, VotesError::ProposalStillActive)
                }
                if p.inner.status != PropStatus::Running {
                   panic_with_error!(env, VotesError::ProposalNotRunning)
                }
                p.inner.status = if p.in_favor > p.against && min_threshold_configuration < (p.in_favor + p.against) {
                    PropStatus::Accepted
                } else {
                    PropStatus::Rejected
                };

                env.storage().set(&ArchiveKey(proposal_id), &p.inner);

                // return reserved tokens
                let core_id = env.storage().get_unchecked(&CORE).unwrap();
                let core = CoreContractClient::new(&env, &core_id);
                let native_asset_id = core.get_native_asset_id();
                let native_token = token::Client::new(&env, &native_asset_id);
                let contract = env.current_contract_address();
                native_token.transfer(&contract, &p.inner.owner, &RESERVE_AMOUNT);

                active_proposals.set(i as u32, p.clone());
                env.storage().set(&key, &active_proposals);
                env.events().publish(
                    (PROPOSAL, STATUS_UPDATE),
                    ProposalStatusUpdateEventData {
                        proposal_id,
                        status: p.inner.status,
                    },
                );
                return;
            }
        }
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn mark_implemented(env: &Env, proposal_id: u32) {
        let key = ArchiveKey(proposal_id);
        let mut proposal: Proposal = env.storage().get_unchecked(&key).unwrap();

        if proposal.status != PropStatus::Accepted {
            panic_with_error!(env, VotesError::UnacceptedProposal)
        }

        proposal.status = PropStatus::Implemented;

        env.storage().set(&key, &proposal);
        env.events().publish(
            (PROPOSAL, STATUS_UPDATE),
            ProposalStatusUpdateEventData {
                proposal_id,
                status: proposal.status,
            },
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    pub url: Bytes,
    pub hash: Bytes,
}

#[contracttype]
struct KeyMeta(u32);

impl Metadata {
    pub fn set(
        env: &Env,
        dao_id: Bytes,
        proposal_id: u32,
        url: Bytes,
        hash: Bytes,
        owner: Address,
    ) -> Self {
        owner.require_auth();

        let key = ActiveKey(dao_id);
        let active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for p in active_proposals.iter_unchecked() {
            if p.id == proposal_id {
                if p.inner.owner != owner {
                    panic_with_error!(env, VotesError::NotProposalOwner)
                }
                let meta = Metadata { url, hash };
                env.storage().set(&KeyMeta(proposal_id), &meta);
                return meta;
            }
        }
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn get(env: &Env, proposal_id: u32) -> Self {
        let key = KeyMeta(proposal_id);
        if !env.storage().has(&key) {
            panic_with_error!(env, VotesError::MetadataNotFound)
        }
        env.storage().get_unchecked(&key).unwrap()
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Configuration {
    pub proposal_duration: u32,
    pub proposal_token_deposit: u128,
    pub min_threshold_configuration: i128,
    pub voting: Voting,
}

impl Configuration {
    pub fn set(
        env: &Env,
        dao_id: Bytes,
        proposal_duration: u32,
        proposal_token_deposit: u128,
        min_threshold_configuration: i128,
        voting: Voting,
    ) -> Self {
        let configuration = Configuration {
            proposal_duration,
            proposal_token_deposit,
            min_threshold_configuration,
            voting,
        };
        env.storage().set(&dao_id, &configuration);
        configuration
    }

    pub fn get(env: &Env, dao_id: Bytes) -> Self {
        if !env.storage().has(&dao_id) {
            panic_with_error!(env, VotesError::ConfigurationNotFound)
        }
        env.storage().get_unchecked(&dao_id).unwrap()
    }
}