use soroban_sdk::{contracttype, Address, Bytes, Symbol, symbol_short};

use crate::types::{PropStatus};

pub const CORE: Symbol = symbol_short!("CORE");
pub const PROPOSAL: Symbol = symbol_short!("PROPOSAL");

pub const CREATED: Symbol = symbol_short!("created");
pub const METADATA_SET: Symbol = symbol_short!("meta_set");
pub const VOTE_CAST: Symbol = symbol_short!("vote_cast");
pub const FAULTED: Symbol = symbol_short!("faulted");
pub const STATUS_UPDATE: Symbol = symbol_short!("state_upd");
pub const CONF_SET: Symbol = symbol_short!("conf_set");

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
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteCastEventData {
    pub proposal_id: u32,
    pub voter_id: Address,
    pub in_favor: bool,
    pub voting_power: i128
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
