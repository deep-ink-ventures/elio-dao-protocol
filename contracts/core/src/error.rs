use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum CoreError {
    DaoAlreadyExists = 0,
    DaoDoesNotExist = 1,
    VotesAlreadyInitiated = 2,
    NotDaoOwner = 3,
    AssetAlreadyIssued = 4,
    AssetNotIssued = 5,
    NoMetadata = 6,
    NoHookpoint = 7,
    MustRemoveConfigFirst = 8,
}