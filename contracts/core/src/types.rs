use soroban_sdk::{contracttype, Bytes, Symbol, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dao {
    pub id: Symbol,
    pub name: Bytes,
    pub owner: Address,
    // todo: How to add Option<Address> style types
}

impl Dao {
    /// Create a new dao for the owner
    pub fn create(env: &Env, id: Symbol, name: Bytes, owner: Address) -> Self {
        if !Self::exists(&env, &id) {
            let dao = Dao { id: id.clone(), name, owner };
            env.storage().set(&id, &dao);
            dao
        } else {
            panic!("DAO already exists")
        }
    }
    
    /// Loads the DAO
    pub fn load(env: &Env, dao_id: &Symbol) -> Self {
        if !Self::exists(env, dao_id) {
            panic!("DAO does not exists")
        }
        env.storage().get_unchecked(dao_id).unwrap()
    }
    
    /// Loads the DAO but with checks for the owner
    pub fn load_for_owner(env: &Env, dao_id: &Symbol, dao_owner: &Address) -> Self {
        dao_owner.require_auth();
        
        let dao = Self::load(&env, dao_id);
        if dao_owner != &dao.owner {
            panic!("Address not DAO Owner")
        }
        dao
    }
    
    /// Checks if a DAO exists
    pub fn exists(env: &Env, dao_id: &Symbol) -> bool {
        env.storage().has(dao_id)
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