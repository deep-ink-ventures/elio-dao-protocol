use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VotesError {
    CoreAlreadyInitialized = 1,
    NotDaoOwner = 2,
    MaxProposalsReached = 3,
    ProposalNotFound = 4,
    ProposalStillActive = 5,
    ProposalNotRunning = 6,
    UnacceptedProposal = 7,
    NotProposalOwner = 8,
    MetadataNotFound = 9,
    ConfigurationNotFound = 10,
}