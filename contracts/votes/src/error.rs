use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VotesError {
    CoreAlreadyInitialized = 0,
    NotDaoOwner = 1,
    MaxProposalsReached = 2,
    ProposalNotFound = 3,
    ProposalStillActive = 4,
    ProposalNotRunning = 5,
    UnacceptedProposal = 6,
    NotProposalOwner = 7,
    MetadataNotFound = 8,
    ConfigurationNotFound = 9,
}