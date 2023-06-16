use soroban_sdk::{contracttype, log, Address, Bytes, Env, IntoVal, Symbol, Vec};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProposalId(u32);

#[contracttype]
struct ActiveKey(Bytes);

#[contracttype]
struct ArchiveKey(ProposalId);

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
pub const FINALIZATION_DURATION: u32 = 5_000;
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
            id: ProposalId(id),
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
        ProposalId(id)
    }

    pub fn get_active(env: &Env, dao_id: Bytes) -> Vec<ActiveProposal> {
        let key = ActiveKey(dao_id);
        if !env.storage().has(&key) {
            return Vec::new(env);
        }
        let active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        let mut filtered_proposals: Vec<ActiveProposal> = Vec::new(env);

        // filter out outdated proposals
        let len = active_proposals.len();
        for proposal in active_proposals.into_iter_unchecked() {
            if env.ledger().sequence()
                <= proposal.inner.ledger + PROPOSAL_DURATION + FINALIZATION_DURATION
            {
                filtered_proposals.push_back(proposal);
            }
        }
        if filtered_proposals.len() < len {
            env.storage().set(&key, &filtered_proposals);
        }

        filtered_proposals
    }

    pub fn get_archived(env: &Env, proposal_id: ProposalId) -> Proposal {
        let key = ArchiveKey(proposal_id);
        env.storage().get_unchecked(&key).unwrap()
    }

    pub fn vote(
        env: &Env,
        dao_id: Bytes,
        proposal_id: ProposalId,
        in_favor: bool,
        voter: Address,
        asset_id: Address,
    ) {
        let key = ActiveKey(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                log!(env, "getting voting power");
                // let voting_power = asset.get_balance_at(&voter, &p.inner.ledger);
                let voting_power: i128 = env.invoke_contract(
                    &asset_id,
                    &Symbol::new(env, "get_balance_at"),
                    (voter, p.inner.ledger).into_val(env),
                );

                if in_favor {
                    p.in_favor += voting_power;
                } else {
                    p.against += voting_power;
                }
                log!(env, "updating proposal votes");
                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);
                return;
            }
        }
        panic!("proposal not found");
    }

    pub fn set_faulty(env: &Env, dao_id: Bytes, proposal_id: ProposalId, reason: Bytes) {
        let key = ActiveKey(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                p.inner.status = PropStatus::Faulty(reason);

                log!(env, "updating proposal");
                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);
                return;
            }
        }
        panic!("proposal not found");
    }

    pub fn finalize(env: &Env, dao_id: Bytes, proposal_id: ProposalId) {
        let key = ActiveKey(dao_id);
        let mut active_proposals: Vec<ActiveProposal> = env.storage().get_unchecked(&key).unwrap();
        for (i, mut p) in active_proposals.iter_unchecked().enumerate() {
            if p.id == proposal_id {
                if env.ledger().sequence() <= p.inner.ledger + PROPOSAL_DURATION {
                    panic!("proposal still active");
                }
                if p.inner.status != PropStatus::Running {
                    panic!("proposal is not running");
                }
                p.inner.status = if p.in_favor > p.against {
                    PropStatus::Accepted
                } else {
                    PropStatus::Rejected
                };

                log!(env, "archiving proposal");
                env.storage().set(&ArchiveKey(proposal_id), &p.inner);

                log!(env, "updating proposal");
                active_proposals.set(i as u32, p);
                env.storage().set(&key, &active_proposals);

                return;
            }
        }
        panic!("proposal not found");
    }

    pub fn mark_implemented(env: &Env, proposal_id: ProposalId) {
        let key = ArchiveKey(proposal_id);
        let mut proposal: Proposal = env.storage().get_unchecked(&key).unwrap();

        if proposal.status != PropStatus::Accepted {
            panic!("proposal was not accepted");
        }

        proposal.status = PropStatus::Implemented;

        env.storage().set(&key, &proposal);
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

        let key = ActiveKey(dao_id);
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
