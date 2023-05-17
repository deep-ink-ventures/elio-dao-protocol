use soroban_sdk::{contracttype, Address, Bytes, Env, Symbol, Vec};

pub type ProposalId = u32;

#[contracttype]
struct KeyActive(Bytes);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub dao_id: Bytes,
    pub ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveProposal {
    pub id: ProposalId,
    pub inner: Proposal,
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
            inner: Proposal {
                dao_id: dao_id.clone(),
                ledger: env.ledger().sequence(),
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
