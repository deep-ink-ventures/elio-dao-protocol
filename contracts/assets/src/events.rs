use soroban_sdk::{contracttype, Address, Bytes, Symbol};

pub const ASSET: Symbol = Symbol::short("ASSET");

pub const MINTED: Symbol = Symbol::short("minted");
pub const OWNER_CHANGED: Symbol = Symbol::short("new_owner");
pub const GOVERNANCE_ID_CHANGED: Symbol = Symbol::short("new_govid");
pub const TRANSFERRED: Symbol = Symbol::short("transfer");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetCreatedEventData {
    pub dao_id: Bytes,
    pub owner_id: Address,
    pub governance_id: Address,
    pub name: Bytes,
    pub symbol: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetMintedEventData {
    pub owner_id: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetNewOwnerEventData {
    pub new_owner_id: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetSetGovernanceIDEventData {
    pub governance_id: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetTransferredEventData {
    pub owner_id: Address,
    pub new_owner_id: Address,
    pub amount: i128,
}
