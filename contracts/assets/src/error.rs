use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AssetError {
    NegativeAmount = 0,
    CheckpointIndexError = 1,
    InsufficientAllowance = 2,
    DaoAlreadyIssuedToken = 3,
    NotTokenOwner = 4,
    CanOnlyBeMintedOnce = 5,
    InsufficientBalance = 6,
    NoCheckpoint = 7
}