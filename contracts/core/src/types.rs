use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, IntoVal, panic_with_error, symbol_short};

use crate::events::{AssetCreatedEventData, ASSET, CREATED};
use crate::error::CoreError;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dao {
    pub id: Bytes,
    pub name: Bytes,
    pub owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metadata {
    pub url: Bytes,
    pub hash: Bytes,
}

#[derive(Clone)]
#[contracttype]
pub enum DaoArtifact {
    Metadata(Bytes),
    Asset(Bytes),
    Hookpoint(Bytes)
}

impl Dao {
    /// Create a new dao for the owner
    pub fn create(env: &Env, id: Bytes, name: Bytes, owner: Address) -> Self {
        if Self::exists(env, &id) {
            panic_with_error!(env, CoreError::DaoAlreadyExists)
        }
        let dao = Dao { id, name, owner };
        env.storage().persistent().set(&dao.id, &dao);
        dao
    }

    /// Loads the DAO
    pub fn load(env: &Env, id: &Bytes) -> Self {
        if !Self::exists(env, id) {
            panic_with_error!(env, CoreError::DaoDoesNotExist)
        }
        env.storage().persistent().get(id).unwrap()
    }

    /// Loads the DAO but with checks for the owner
    pub fn load_for_owner(env: &Env, id: &Bytes, owner: &Address) -> Self {
        owner.require_auth();

        let dao = Self::load(env, id);
        if owner != &dao.owner {
            panic_with_error!(env, CoreError::NotDaoOwner)
        }
        dao
    }

    /// Checks if a DAO exists
    pub fn exists(env: &Env, id: &Bytes) -> bool {
        env.storage().persistent().has(id)
    }

    /// +++ Member functions +

    pub fn issue_token(self, env: &Env, assets_wasm_hash: BytesN<32>, asset_salt: BytesN<32>) -> Address {
        let key = DaoArtifact::Asset(self.id.clone());

        if env.storage().persistent().has(&key) {
            panic_with_error!(env, CoreError::AssetAlreadyIssued)
        }

        let asset_id = env
            .deployer()
            .with_current_contract(asset_salt)
            .deploy(assets_wasm_hash);

        env.storage().persistent().set(&key, &asset_id);

        let init_fn = symbol_short!("init");

        let core_address = env.current_contract_address();
        let init_args = (
            self.id.clone(),
            self.name,
            self.owner.clone(),
            core_address.clone(),
        )
            .into_val(env);
        env.invoke_contract::<()>(&asset_id, &init_fn, init_args);

        env.events().publish(
            (ASSET, CREATED, self.id.clone()),
            AssetCreatedEventData {
                dao_id: self.id,
                asset_id: asset_id.clone(),
                owner_id: self.owner,
            },
        );
        asset_id
    }

    pub fn get_asset_id(&self, env: &Env) -> Address {
        let key = DaoArtifact::Asset(self.id.clone());
        if !env.storage().persistent().has(&key) {
            panic_with_error!(env, CoreError::AssetNotIssued)
        }
        env.storage().persistent().get(&key).unwrap()
    }

    /// Destroys a dao
    pub fn destroy(&self, env: &Env) {
        env.storage().persistent().remove(&self.id);
    }

    /// Saves a dao
    pub fn save(&self, env: &Env) {
        env.storage().persistent().set(&self.id, self);
    }
}

impl Metadata {
    /// Create metadata for the dao
    pub fn create(env: &Env, dao_id: Bytes, url: Bytes, hash: Bytes) -> Self {
        let meta = Metadata { url, hash };
        env.storage().persistent().set(&DaoArtifact::Metadata(dao_id), &meta);
        meta
    }

    /// Loads the metadata
    pub fn load(env: &Env, dao_id: &Bytes) -> Self {
        if !Self::exists(env, dao_id) {
            panic_with_error!(env, CoreError::NoMetadata)
        }
        env.storage().persistent().get(&DaoArtifact::Metadata(dao_id.clone())).unwrap()
    }

    /// Checks if metadata for the dao_id exists
    pub fn exists(env: &Env, dao_id: &Bytes) -> bool {
        env.storage().persistent().has(&DaoArtifact::Metadata(dao_id.clone()))
    }
}
