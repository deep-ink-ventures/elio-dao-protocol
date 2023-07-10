use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AssetError {
    NegativeAmount = 1,
    CheckpointIndexError = 2,
    InsufficientAllowance = 3,
    DaoAlreadyIssuedToken = 4,
    NotTokenOwner = 5,
    CanOnlyBeMintedOnce = 6,
    InsufficientBalance = 7
}