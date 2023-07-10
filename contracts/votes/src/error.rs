use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VotesError {
    CoreAlreadyInitialized = 1000,
    NotDaoOwner = 1001,
    MaxProposalsReached = 1002,
    ProposalNotFound = 1003,
    ProposalStillActive = 1004,
    ProposalNotRunning = 1005,
    UnacceptedProposal = 1006,
    NotProposalOwner = 1007,
    MetadataNotFound = 1008,
    ConfigurationNotFound = 1009,
}