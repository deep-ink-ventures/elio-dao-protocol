#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, BytesN, Env};

mod test;

mod events;
mod interface;
use events::{
    DaoCreatedEventData, DaoDestroyedEventData, DaoMetadataSetEventData, DaoOwnerChangedEventData,
    CREATED, DAO, DESTROYED, METADATA_SET, OWNER_CHANGED, VOTES,
};
use interface::CoreTrait;

mod types;
use types::{Dao, Metadata};

pub struct CoreContract;

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
        let dao = Dao::create(&env, dao_id.clone(), dao_name.clone(), dao_owner.clone());
        env.events().publish(
            (DAO, CREATED),
            DaoCreatedEventData {
                dao_id,
                dao_name,
                owner_id: dao_owner,
            },
        );
        dao
    }

    fn get_dao(env: Env, dao_id: Bytes) -> Dao {
        Dao::load(&env, &dao_id)
    }

    fn destroy_dao(env: Env, dao_id: Bytes, dao_owner: Address) {
        Dao::load_for_owner(&env, &dao_id, &dao_owner).destroy(&env);

        // todo: release reserve
        env.events()
            .publish((DAO, DESTROYED), DaoDestroyedEventData { dao_id });
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
        let meta = Metadata::create(&env, dao_id.clone(), url.clone(), hash.clone());
        env.events().publish(
            (DAO, METADATA_SET),
            DaoMetadataSetEventData { dao_id, url, hash },
        );
        meta
    }

    fn get_metadata(env: Env, dao_id: Bytes) -> Metadata {
        Metadata::load(&env, &dao_id)
    }

    fn change_owner(env: Env, dao_id: Bytes, new_owner: Address, dao_owner: Address) -> Dao {
        let mut dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        dao.owner = new_owner.clone();
        dao.save(&env);
        env.events().publish(
            (DAO, OWNER_CHANGED),
            DaoOwnerChangedEventData {
                dao_id,
                new_owner_id: new_owner,
            },
        );
        dao
    }
}
