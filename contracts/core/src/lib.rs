#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, BytesN, Env, Symbol};

mod test;

mod interface;
use interface::CoreTrait;

mod types;
use types::{Dao, Metadata};

pub struct CoreContract;

const DAO: Symbol = Symbol::short("DAO");
const VOTES: Symbol = Symbol::short("VOTES");

#[contractimpl]
impl CoreTrait for CoreContract {
    fn init(env: Env, votes_id: BytesN<32>) {
        if env.storage().has(&VOTES) {
            panic!("Already initialized")
        }
        env.storage().set(&VOTES, &votes_id);
    }

    fn get_votes_id(env: Env) -> BytesN<32> {
        env.storage().get_unchecked(&VOTES).unwrap()
    }

    fn create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao {
        // todo: reserve
        let dao = Dao::create(&env, dao_id.clone(), dao_name, dao_owner);
        env.events()
            .publish((DAO, Symbol::short("created")), dao.clone());
        dao
    }

    fn get_dao(env: Env, dao_id: Bytes) -> Dao {
        Dao::load(&env, &dao_id)
    }

    fn destroy_dao(env: Env, dao_id: Bytes, dao_owner: Address) {
        Dao::load_for_owner(&env, &dao_id, &dao_owner).destroy(&env);

        // todo: release reserve
        env.events()
            .publish((DAO, Symbol::short("destroyed")), dao_id.clone());
    }

    fn issue_token(
        env: Env,
        dao_id: Bytes,
        dao_owner: Address,
        assets_wasm_hash: BytesN<32>,
        asset_salt: Bytes,
    ) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        dao.issue_token(&env, assets_wasm_hash, asset_salt);
    }

    fn get_dao_asset_id(env: Env, dao_id: Bytes) -> BytesN<32> {
        Dao::load(&env, &dao_id).get_asset_id(&env)
    }

    fn set_metadata(
        env: Env,
        dao_id: Bytes,
        url: Bytes,
        hash: Bytes,
        dao_owner: Address,
    ) -> Metadata {
        // this is to load & verify ownership
        Dao::load_for_owner(&env, &dao_id, &dao_owner);
        let meta = Metadata::create(&env, dao_id, url, hash);
        env.events()
            .publish((DAO, Symbol::short("meta_set")), meta.clone());
        meta
    }

    fn get_metadata(env: Env, dao_id: Bytes) -> Metadata {
        Metadata::load(&env, &dao_id)
    }

    fn change_owner(env: Env, dao_id: Bytes, new_owner: Address, dao_owner: Address) -> Dao {
        let mut dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        dao.owner = new_owner;
        dao.save(&env);
        env.events()
            .publish((DAO, Symbol::short("new_owner")), dao.clone());
        dao
    }
}
