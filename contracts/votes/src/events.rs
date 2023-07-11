use soroban_sdk::{contracttype, Address, Bytes, Symbol};

use crate::types::{Voting, PropStatus};

pub const CORE: Symbol = Symbol::short("CORE");
pub const PROPOSAL: Symbol = Symbol::short("PROPOSAL");

pub const CREATED: Symbol = Symbol::short("created");
pub const METADATA_SET: Symbol = Symbol::short("meta_set");
pub const VOTE_CAST: Symbol = Symbol::short("vote_cast");
pub const FAULTED: Symbol = Symbol::short("faulted");
pub const STATUS_UPDATE: Symbol = Symbol::short("state_upd");
pub const CONF_SET: Symbol = Symbol::short("conf_set");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCreatedEventData {
    pub proposal_id: u32,
    pub dao_id: Bytes,
    pub owner_id: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalMetadataSetEventData {
    pub proposal_id: u32,
    pub url: Bytes,
    pub hash: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalConfigurationSetEventData {
    pub dao_id: Bytes,
    pub proposal_duration: u32,
    pub proposal_token_deposit: u128,
    pub proposal_voting_type: Voting,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteCastEventData {
    pub proposal_id: u32,
    pub voter_id: Address,
    pub in_favor: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalFaultedEventData {
    pub proposal_id: u32,
    pub reason: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalStatusUpdateEventData {
    pub proposal_id: u32,
    pub status: PropStatus,
}
