use soroban_sdk::{contracttype, Address, Bytes, Symbol};

use crate::types::ProposalId;

pub const CORE: Symbol = Symbol::short("CORE");
pub const PROPOSAL: Symbol = Symbol::short("PROPOSAL");

pub const CREATED: Symbol = Symbol::short("created");
pub const METADATA_SET: Symbol = Symbol::short("meta_set");
pub const VOTE_CAST: Symbol = Symbol::short("vote_cast");
pub const FAULTED: Symbol = Symbol::short("faulted");
pub const FINALIZED: Symbol = Symbol::short("finalized");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalCreatedEventData {
    pub proposal_id: ProposalId,
    pub dao_id: Bytes,
    pub owner_id: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalMetadataSetEventData {
    pub proposal_id: ProposalId,
    pub url: Bytes,
    pub hash: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteCastEventData {
    pub proposal_id: ProposalId,
    pub voter_id: Address,
    pub in_favor: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalFaultedEventData {
    pub proposal_id: ProposalId,
    pub reason: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposalFinalizedEventData {
    pub proposal_id: ProposalId,
}
