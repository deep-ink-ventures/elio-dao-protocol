use soroban_sdk::{contracttype, Address, Bytes, Env, IntoVal, Symbol, Vec, token, panic_with_error, symbol_short};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}
use core_contract::Client as CoreContractClient;

use crate::error::VotesError;

use crate::events::{ProposalStatusUpdateEventData, STATUS_UPDATE, PROPOSAL, CORE};
use crate::hooks::{on_vote, on_before_proposal_creation, on_before_set_metadata, on_set_configuration, on_before_fault_proposal, on_before_finalize_proposal};

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
    Majority,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VotingHistory {
    Voting(Address, u32),
}

pub const XLM: i128 = 10_000_000;
pub const RESERVE_AMOUNT: i128 = 100 * XLM;
pub const PROPOSAL_MAX_NR: u32 = 25;

pub const A_WEEK_IN_LEDGERS: u32 = 100800;
pub const BUMP_A_MONTH: u32 = 432000;
pub const BUMP_A_MONTH_THRESHOLD: u32 = 432000 - A_WEEK_IN_LEDGERS;

const PROP_ID: Symbol = symbol_short!("PROP_ID");

impl Proposal {

    pub fn create(env: &Env, dao_id: Bytes, owner: Address, core_id: Address) -> u32 {
        owner.require_auth();

        let mut proposals = Self::get_active(env, dao_id.clone());
        if proposals.len() == PROPOSAL_MAX_NR {
            panic_with_error!(env, VotesError::MaxProposalsReached)
        }

        on_before_proposal_creation(env, &dao_id, &owner);

        // Transfer required amount to prevent spam
        let core = CoreContractClient::new(env, &core_id);
        let native_asset_id = core.get_native_asset_id();
        let native_token = token::Client::new(env, &native_asset_id);
        let contract = env.current_contract_address();
        native_token.transfer(&owner, &contract, &RESERVE_AMOUNT);

        let id = env.storage().instance().get(&PROP_ID).unwrap_or(0);
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
        let key = ActiveKey(dao_id.clone());

        env.storage().persistent().set(&key, &proposals);
        env.storage().instance().set(&PROP_ID, &(id + 1));

        env.storage().instance().bump(BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH + Configuration::get(env, dao_id).proposal_duration);
        id
    }

    pub fn get_active(env: &Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        let key = ActiveKey(dao_id.clone());
        if !env.storage().persistent().has(&key) {
            return Vec::new(env);
        }
        let active_proposals: Vec<ActiveProposal> = env.storage().persistent().get(&key).unwrap();
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH + Configuration::get(env, dao_id.clone()).proposal_duration);
        let mut filtered_proposals: Vec<ActiveProposal> = Vec::new(env);

        let proposal_duration = Configuration::get(env, dao_id).proposal_duration;

        // filter out outdated proposals
        let len = active_proposals.len();
        for proposal in active_proposals.into_iter() {
            if env.ledger().sequence() <= proposal.inner.ledger + proposal_duration {
                filtered_proposals.push_back(proposal);
            }
        }
        if filtered_proposals.len() < len {
            env.storage().persistent().set(&key, &filtered_proposals);
        }

        filtered_proposals
    }

    pub fn get_archived(env: &Env, proposal_id: u32) -> Proposal {
        let key = ArchiveKey(proposal_id);
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        env.storage().persistent().get(&key).unwrap()

    }

    pub fn vote(
        env: &Env,
        dao_id: Bytes,
        proposal_id: u32,
        in_favor: bool,
        voter: Address,
        asset_id: Address,
    ) -> i128 {
        // Check if voter has already voted and has the same vote.
        let vote_key = VotingHistory::Voting(voter.clone(), proposal_id);
        let has_key = env.storage().temporary().has(&vote_key);
        if has_key && in_favor == env.storage().temporary().get::<VotingHistory, bool>(&vote_key).unwrap() {
            panic_with_error!(env, VotesError::VoteAlreadyCast)
        }
        let key = ActiveKey(dao_id.clone());
        let mut active_proposals: Vec<ActiveProposal> = env.storage().persistent().get(&key).unwrap();
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH + Configuration::get(env, dao_id.clone()).proposal_duration);

        for (i, mut p) in active_proposals.clone().into_iter().enumerate() {
            if p.id == proposal_id {
                let voting_power_pre_hook: i128 = env.invoke_contract(
                    &asset_id,
                    &Symbol::new(env, "get_balance_at"),
                    (voter.clone(), p.inner.ledger).into_val(env),
                );
                let voting_power = on_vote(env, &dao_id, &proposal_id, &voter, voting_power_pre_hook);

                if in_favor {
                    p.in_favor += voting_power;
                    if has_key { p.against -= voting_power}
                } else {
                    p.against += voting_power;
                    if has_key { p.in_favor -= voting_power}
                }
                active_proposals.set(i as u32, p);
                env.storage().persistent().set(&key, &active_proposals);
                env.storage().temporary().set(&vote_key, &in_favor);
                env.storage().temporary().bump(&vote_key, 0, Configuration::get(env, dao_id.clone()).proposal_duration);
                return voting_power
            }
        }
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn set_faulty(env: &Env, dao_id: Bytes, proposal_id: u32, reason: Bytes) {
        on_before_fault_proposal(env, &dao_id, proposal_id, &reason);
        let key = ActiveKey(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().persistent().get(&key).unwrap();
        for (i, mut p) in active_proposals.clone().into_iter().enumerate() {
            if p.id == proposal_id {
                p.inner.status = PropStatus::Faulty(reason);

                // return reserved tokens
                let core_id = env.storage().instance().get(&CORE).unwrap();
                let core = CoreContractClient::new(env, &core_id);
                let native_asset_id = core.get_native_asset_id();
                let native_token = token::Client::new(env, &native_asset_id);
                let contract = env.current_contract_address();
                native_token.transfer(&contract, &p.inner.owner, &RESERVE_AMOUNT);

                active_proposals.set(i as u32, p);
                env.storage().persistent().set(&key, &active_proposals);
                return;
            }
        }
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn finalize(env: &Env, dao_id: Bytes, proposal_id: u32) {
        on_before_finalize_proposal(env, &dao_id, proposal_id);
        let key = ActiveKey(dao_id.clone());
        let configuration = Configuration::get(env, dao_id);
        let proposal_duration = configuration.proposal_duration;
        let min_threshold_configuration = configuration.min_threshold_configuration;
        let mut active_proposals: Vec<ActiveProposal> = env.storage().persistent().get(&key).unwrap();
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);

        for (i, mut p) in active_proposals.clone().into_iter().enumerate() {
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

                env.storage().persistent().set(&ArchiveKey(proposal_id), &p.inner);

                // return reserved tokens
                let core_id = env.storage().instance().get(&CORE).unwrap();
                let core = CoreContractClient::new(env, &core_id);
                let native_asset_id = core.get_native_asset_id();
                let native_token = token::Client::new(env, &native_asset_id);
                let contract = env.current_contract_address();
                native_token.transfer(&contract, &p.inner.owner, &RESERVE_AMOUNT);

                active_proposals.set(i as u32, p.clone());
                env.storage().persistent().set(&key, &active_proposals);
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
        let mut proposal: Proposal = env.storage().persistent().get(&key).unwrap();
        env.storage().persistent().bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);

        if proposal.status != PropStatus::Accepted {
            panic_with_error!(env, VotesError::UnacceptedProposal)
        }

        proposal.status = PropStatus::Implemented;

        env.storage().persistent().set(&key, &proposal);
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

        let key = ActiveKey(dao_id.clone());
        let active_proposals: Vec<ActiveProposal> = env.storage().persistent().get(&key).unwrap();
        for p in active_proposals.into_iter() {
            if p.id == proposal_id {
                if p.inner.owner != owner {
                    panic_with_error!(env, VotesError::NotProposalOwner)
                }
                if env.storage().persistent().has(&KeyMeta(proposal_id)) {
                    panic_with_error!(env, VotesError::MetadataAlreadySet)
                }
                on_before_set_metadata(env, &dao_id, proposal_id, &url, &hash, &p.inner.owner);
                let meta = Metadata { url, hash };
                env.storage().persistent().set(&KeyMeta(proposal_id), &meta);
                return meta;
            }
        }
        panic_with_error!(env, VotesError::ProposalNotFound)
    }

    pub fn get(env: &Env, proposal_id: u32) -> Self {
        let key = KeyMeta(proposal_id);
        if !env.storage().persistent().has(&key) {
            panic_with_error!(env, VotesError::MetadataNotFound)
        }
        env.storage().persistent().get(&key).unwrap()
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Configuration {
    pub proposal_duration: u32,
    pub min_threshold_configuration: i128,
}

impl Configuration {
    pub fn set(
        env: &Env,
        dao_id: Bytes,
        proposal_duration: u32,
        min_threshold_configuration: i128,
    ) -> Self {
        let configuration = Configuration {
            proposal_duration,
            min_threshold_configuration,
        };
        env.storage().persistent().set(&dao_id, &configuration);
        on_set_configuration(env, &dao_id, proposal_duration);
        configuration
    }

    pub fn get(env: &Env, dao_id: Bytes) -> Self {
        if !env.storage().persistent().has(&dao_id) {
            panic_with_error!(env, VotesError::ConfigurationNotFound)
        }
        env.storage().persistent().get(&dao_id).unwrap()
    }

    pub fn remove(env: &Env, dao_id: Bytes) {
        env.storage().persistent().remove(&dao_id)
    }
}