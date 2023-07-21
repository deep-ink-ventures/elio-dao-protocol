use soroban_sdk::{contracttype, Address, Bytes, Symbol, symbol_short};

pub const ASSET: Symbol = symbol_short!("ASSET");

pub const MINTED: Symbol = symbol_short!("minted");
pub const OWNER_CHANGED: Symbol = symbol_short!("new_owner");
pub const CORE_ADDRESS_CHANGED: Symbol = symbol_short!("new_govid");
pub const TRANSFERRED: Symbol = symbol_short!("transfer");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetCreatedEventData {
    pub dao_id: Bytes,
    pub owner_id: Address,
    pub core_address: Address,
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
    pub core_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetTransferredEventData {
    pub owner_id: Address,
    pub new_owner_id: Address,
    pub amount: i128,
}
