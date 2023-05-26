use soroban_sdk::{contracttype, log, Address, Bytes, Env, Symbol, Vec};

use crate::assets_contract;

pub type ProposalId = u32;

#[contracttype]
struct KeyActive(Bytes);

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
    pub id: ProposalId,
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

const PROP_ID: Symbol = Symbol::short("PROP_ID");

pub const PROPOSAL_DURATION: u32 = 10_000;
pub const PROPOSAL_MAX_NR: u32 = 25;

impl Proposal {
    pub fn create(env: &Env, dao_id: Bytes, owner: Address) -> ProposalId {
        owner.require_auth();

        let mut proposals = Self::get_active(env, dao_id.clone());
        if proposals.len() == PROPOSAL_MAX_NR {
            panic!("already at maximum number of {PROPOSAL_MAX_NR} proposals");
        }

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
        env.storage().set(&KeyActive(dao_id), &proposals);
        env.storage().set(&PROP_ID, &(id + 1));
        id
    }

    pub fn get_active(env: &Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        let key = KeyActive(dao_id);
        if !env.storage().has(&key) {
            return Vec::new(env);
        }
        let active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        let mut filtered_proposals: Vec<ActiveProposal> = Vec::new(env);

        // filter out outdated proposals
        let len = active_proposals.len();
        for proposal in active_proposals.into_iter_unchecked() {
            if env.ledger().sequence() <= proposal.inner.ledger + PROPOSAL_DURATION {
                filtered_proposals.push_back(proposal);
            }
        }
        if filtered_proposals.len() < len {
            env.storage().set(&key, &filtered_proposals);
        }

        filtered_proposals
    }
}

impl ActiveProposal {
    pub fn vote(
        env: Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        in_favor: bool,
        voter: Address,
        asset: assets_contract::Client,
    ) {
        let key = KeyActive(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                log!(&env, "getting voting power");
                let voting_power = asset.get_balance_at(&voter, &p.inner.ledger);

                if in_favor {
                    p.in_favor += voting_power;
                } else {
                    p.against += voting_power;
                }
                log!(&env, "updating proposal votes");
                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);
                return;
            }
        }
        panic!("proposal not found");
    }

    pub fn set_faulty(env: Env, dao_id: Bytes, proposal_id: ProposalId, reason: Bytes) {
        let key = KeyActive(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                p.inner.status = PropStatus::Faulty(reason);
                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);
                return;
            }
        }
        panic!("proposal not found");
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    pub url: Bytes,
    pub hash: Bytes,
}

#[contracttype]
struct KeyMeta(ProposalId);

impl Metadata {
    pub fn set(
        env: &Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        url: Bytes,
        hash: Bytes,
        owner: Address,
    ) -> Self {
        owner.require_auth();

        let key = KeyActive(dao_id);
        let active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for p in active_proposals.iter_unchecked() {
            if p.id == proposal_id {
                if p.inner.owner != owner {
                    panic!("only the owner can set metadata");
                }
                let meta = Metadata { url, hash };
                env.storage().set(&KeyMeta(proposal_id), &meta);
                return meta;
            }
        }
        panic!("proposal not found");
    }

    pub fn get(env: &Env, proposal_id: ProposalId) -> Self {
        let key = KeyMeta(proposal_id);
        if !env.storage().has(&key) {
            panic!("metadata does not exist");
        }
        env.storage().get_unchecked(&key).unwrap()
    }
}
