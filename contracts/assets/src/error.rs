use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AssetError {
    NegativeAmount = 2000,
    CheckpointIndexError = 2001,
    InsufficientAllowance = 2002,
    DaoAlreadyIssuedToken = 2003,
    NotTokenOwner = 2004,
    CanOnlyBeMintedOnce = 2005,
    InsufficientBalance = 2006
}