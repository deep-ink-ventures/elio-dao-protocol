use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CoreError {
    DaoAlreadyExists = 1,
    DaoDoesNotExist = 2,
    VotesAlreadyInitiated = 3,
    NotDaoOwner = 4,
    AssetAlreadyIssued = 5,
    AssetNotIssued = 6,
    NoMetadata = 7,
    NoHookpoint = 8
}