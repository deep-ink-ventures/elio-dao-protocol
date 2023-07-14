#![no_std]
use soroban_sdk::{contractimpl, contract, token, Address, Bytes, BytesN, Env, Symbol, panic_with_error, symbol_short};

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
use crate::error::CoreError;
use crate::types::DaoArtifact;

mod error;

pub const NATIVE: Symbol = symbol_short!("NATIVE");


const XLM: i128 = 10_000_000;
const RESERVE_AMOUNT: i128 = 1000 * XLM;

#[contract]
pub struct CoreContract;

#[contractimpl]
impl CoreTrait for CoreContract {
    fn init(env: Env, votes_id: Address, native_asset_id: Address) {
        if env.storage().instance().has(&VOTES) {
            panic_with_error!(env, CoreError::VotesAlreadyInitiated)
        }

        env.storage().instance().set(&VOTES, &votes_id);
        env.storage().instance().set(&NATIVE, &native_asset_id);
    }

    fn get_votes_id(env: Env) -> Address {
        env.storage().instance().get(&VOTES).unwrap()
    }

    fn get_native_asset_id(env: Env) -> Address {
        env.storage().instance().get(&NATIVE).unwrap()
    }

    fn create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao {

        // Reserve DAO Tokens
        let native_asset_id = env.storage().instance().get(&NATIVE).unwrap();
        let native_token = token::Client::new(&env, &native_asset_id);
        let contract = &env.current_contract_address();
        native_token.transfer(&dao_owner, &contract, &RESERVE_AMOUNT);

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

       let native_asset_id = env.storage().instance().get(&NATIVE).unwrap();
       let native_token = token::Client::new(&env, &native_asset_id);
       let contract = &env.current_contract_address();
       native_token.transfer(&contract, &dao_owner, &RESERVE_AMOUNT);

        env.events()
            .publish((DAO, DESTROYED), DaoDestroyedEventData { dao_id });
    }

    fn issue_token(
        env: Env,
        dao_id: Bytes,
        dao_owner: Address,
        assets_wasm_hash: BytesN<32>,
        asset_salt: BytesN<32>,
    ) -> Address {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        dao.issue_token(&env, assets_wasm_hash, asset_salt)
    }

    fn get_dao_asset_id(env: Env, dao_id: Bytes) -> Address {
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

    fn has_hookpoint(env: Env, dao_id: Bytes) -> bool {
        env.storage().persistent().has(&DaoArtifact::Hookpoint(dao_id))
    }

    fn get_hookpoint(env: Env, dao_id: Bytes) -> Address {
        if !env.storage().persistent().has(&DaoArtifact::Hookpoint(dao_id.clone())) {
            panic_with_error!(env, CoreError::NoHookpoint)
        }
        env.storage()
            .persistent()
            .get(&DaoArtifact::Hookpoint(dao_id))
            .unwrap()
    }

    fn set_hookpoint(env: Env, dao_id: Bytes, hookpoint: Address, dao_owner: Address) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        env.storage().persistent().set(&DaoArtifact::Hookpoint(dao.id), &hookpoint);
    }

    fn remove_hookpoint(env: Env, dao_id: Bytes, dao_owner: Address) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        if env.storage().persistent().has(&DaoArtifact::Hookpoint(dao_id)) {
            env.storage().persistent().remove(&DaoArtifact::Hookpoint(dao.id))
        }
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
