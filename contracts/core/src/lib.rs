use soroban_sdk::{contractimpl, Env, Symbol, Bytes, Address};

#[cfg(test)]
mod test;

mod interface;
use interface::CoreTrait;

mod types;
use types::{Dao, MetaData};

pub struct CoreContract;

const DAO: Symbol = Symbol::short("DAO");

#[contractimpl]
impl CoreTrait for CoreContract {

    fn create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao  {
        // todo: reserve
        
        let dao = Dao::create(&env, dao_id.clone(), dao_name, dao_owner);
        env.events().publish((DAO, Symbol::short("created")), dao.clone());
        dao
    }

    fn get_dao(env: Env, dao_id: Bytes) -> Dao {
        Dao::load(&env, &dao_id)
    }
    
    fn destroy_dao(env: Env, dao_id: Bytes, dao_owner: Address) {
        Dao::load_for_owner(&env, &dao_id, &dao_owner).destroy(&env);
        
        // todo: release reserve
        env.events().publish((DAO, Symbol::short("destroyed")), dao_id.clone());
    }
    
    fn issue_token(env: Env, dao_id: Bytes, supply: i128, dao_owner: Address) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        // todo: initialize a new asset contract, set the name to the dao name and the id to the dao id
        // todo: mint the initial supply to the dao owner
        // todo: set the address to the asset address
        env.events().publish((DAO, Symbol::short("token_iss")), dao.clone());
    }
    
    fn get_meta_data(env: Env, dao_id: Bytes) -> MetaData {
        MetaData::load(&env, &dao_id)
    }

    fn set_meta_data(env: Env, dao_id: Bytes, url: Bytes, hash: Bytes, dao_owner: Address) -> MetaData {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        let meta = MetaData::create(&env, dao_id, url, hash);
        env.events().publish((DAO, Symbol::short("meta_set")), meta.clone());
        meta
    }

    fn change_owner(env: Env, dao_id: Bytes, new_owner: Address, dao_owner: Address) -> Dao {
        let mut dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        dao.owner = new_owner;
        dao.save(&env);
        env.events().publish((DAO, Symbol::short("new_owner")), dao.clone());
        dao
    }
}