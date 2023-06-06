use soroban_sdk::{contracttype, Address, Bytes, Symbol};

pub const ASSET: Symbol = Symbol::short("ASSET");
pub const DAO: Symbol = Symbol::short("DAO");
pub const VOTES: Symbol = Symbol::short("VOTES");

pub const CREATED: Symbol = Symbol::short("created");
pub const DESTROYED: Symbol = Symbol::short("destroyed");
pub const METADATA_SET: Symbol = Symbol::short("meta_set");
pub const OWNER_CHANGED: Symbol = Symbol::short("new_owner");

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
pub struct AssetEventData {
    pub dao_id: Bytes,
    pub asset_id: Bytes,
    pub owner_id: Address,
}
