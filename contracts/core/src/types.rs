use soroban_sdk::{contracttype, Bytes, Address, Env, BytesN, IntoVal, Symbol};

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
pub enum DaoArticfact {
    Metadata(Bytes),
    Asset(Bytes)
}

impl Dao {
    /// Create a new dao for the owner
    pub fn create(env: &Env, id: Bytes, name: Bytes, owner: Address) -> Self {

        if !Self::exists(&env, &id) {
            let dao = Dao { id: id.clone(), name, owner };
            env.storage().set(&id, &dao);
            dao
        } else {
            panic!("DAO already exists")
        }
    }
    
    /// Loads the DAO
    pub fn load(env: &Env, id: &Bytes) -> Self {
        if !Self::exists(env, id) {
            panic!("DAO does not exists")
        }
        env.storage().get_unchecked(id).unwrap()
    }
    
    /// Loads the DAO but with checks for the owner
    pub fn load_for_owner(env: &Env, id: &Bytes, owner: &Address) -> Self {
        owner.require_auth();
        
        let dao = Self::load(&env, id);
        if owner != &dao.owner {
            panic!("Address not DAO Owner")
        }
        dao
    }
    
    /// Checks if a DAO exists
    pub fn exists(env: &Env, id: &Bytes) -> bool {
        env.storage().has(id)
    }
    
    /// +++ Member functions +
    
    pub fn issue_token(self, env: &Env, assets_wasm_hash: BytesN<32>, asset_salt: Bytes) {
        let key = DaoArticfact::Asset(self.id.clone());
        if env.storage().has(&key) {
            panic!("asset already issued")
        }
        let asset_id = env.deployer().with_current_contract(&asset_salt).deploy(&assets_wasm_hash);
        env.storage().set(&key, &asset_id);

        let init_fn = Symbol::short("init");
        let init_args = (self.id, self.name, self.owner.clone(), env.current_contract_id()).into_val(env);
        env.invoke_contract::<()>(&asset_id, &init_fn, init_args);
    }

    pub fn get_asset_id(&self, env: &Env) -> BytesN<32> {
        let key = DaoArticfact::Asset(self.id.clone());
        if !env.storage().has(&key) {
            panic!("asset not issued")
        }
        env.storage().get_unchecked(&key).unwrap()
    }

    /// Destroys a dao
    pub fn destroy(&self, env: &Env) {
        env.storage().remove(&self.id);
    }
    
    pub fn save(&self, env: &Env) {
        env.storage().set(&self.id, self);
    }
}

impl Metadata {

    /// Create metadata for the dao
    pub fn create(env: &Env, dao_id: Bytes, url: Bytes, hash: Bytes) -> Self {
        let meta = Metadata { url, hash };
        env.storage().set(&DaoArticfact::Metadata(dao_id), &meta);
        meta
    }

    /// Loads the metadata
    pub fn load(env: &Env, dao_id: &Bytes) -> Self {
        if !Self::exists(env, dao_id) {
            panic!("metadata does not exist")
        }
        env.storage().get_unchecked(&DaoArticfact::Metadata(dao_id.clone())).unwrap()
    }

    /// Checks if metadata for the dao_id exists
    pub fn exists(env: &Env, dao_id: &Bytes) -> bool {
        env.storage().has(&DaoArticfact::Metadata(dao_id.clone()))
    }
}
