use soroban_sdk::{contracttype, Bytes, Address, Env, IntoVal};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dao {
    pub id: Bytes,
    pub name: Bytes,
    pub owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetaData {
    pub url: Bytes,
    pub hash: Bytes,
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
    
    /// Destroys a dao
    pub fn destroy(&self, env: &Env) {
        env.storage().remove(&self.id);
    }
    
    pub fn save(&self, env: &Env) {
        env.storage().set(&self.id, self);
    }
    
    pub fn refresh(&self, env: &Env) -> Dao {
        Self::load(env, &self.id)
    }
}

impl MetaData {

    /// Create a unique storage key for the meta data
    fn storage_key(env: &Env, dao_id: &Bytes) -> Bytes {
        let mut prefix: Bytes = "m_".into_val(env);
        prefix.append(dao_id);
        prefix
    }

    /// Create a new metad ata for the dao
    pub fn create(env: &Env, dao_id: Bytes, url: Bytes, hash: Bytes) -> Self {
        let meta = MetaData { url, hash };
        env.storage().set(&Self::storage_key(env, &dao_id), &meta);
        meta
    }

    /// Loads the meta data
    pub fn load(env: &Env, dao_id: &Bytes) -> Self {
        if !Self::exists(env, dao_id) {
            panic!("MetaData does not exists")
        }
        env.storage().get_unchecked(&Self::storage_key(env, &dao_id)).unwrap()
    }

    /// Checks if a meta data for the dao_id exists
    pub fn exists(env: &Env, dao_id: &Bytes) -> bool {
        env.storage().has(&Self::storage_key(env, dao_id))
    }
}