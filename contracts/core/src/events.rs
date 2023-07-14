use soroban_sdk::{contracttype, Address, Bytes, Symbol, symbol_short};

pub const DAO: Symbol = symbol_short!("DAO");
pub const ASSET: Symbol = symbol_short!("ASSET");
pub const VOTES: Symbol = symbol_short!("VOTES");

pub const CREATED: Symbol = symbol_short!("created");
pub const DESTROYED: Symbol = symbol_short!("destroyed");
pub const METADATA_SET: Symbol = symbol_short!("meta_set");
pub const OWNER_CHANGED: Symbol = symbol_short!("new_owner");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaoCreatedEventData {
    pub dao_id: Bytes,
    pub dao_name: Bytes,
    pub owner_id: Address,
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaoDestroyedEventData {
    pub dao_id: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaoMetadataSetEventData {
    pub dao_id: Bytes,
    pub url: Bytes,
    pub hash: Bytes,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DaoOwnerChangedEventData {
    pub dao_id: Bytes,
    pub new_owner_id: Address,
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetCreatedEventData {
    pub dao_id: Bytes,
    pub asset_id: Address,
    pub owner_id: Address,
}