#![no_std]

use soroban_sdk::{contractimpl, token, Address, Bytes, BytesN, Env, Symbol, panic_with_error};

mod votes_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_votes.wasm");
}

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

pub const NATIVE: Symbol = Symbol::short("NATIVE");

pub struct CoreContract;

const XLM: i128 = 10_000_000;
const RESERVE_AMOUNT: i128 = 1000 * XLM;


#[contractimpl]
impl CoreTrait for CoreContract {
    fn init(env: Env, votes_id: Address, native_asset_id: Address) {
        if env.storage().has(&VOTES) {
            panic_with_error!(env, CoreError::VotesAlreadyInitiated)
        }

        env.storage().set(&VOTES, &votes_id);
        env.storage().set(&NATIVE, &native_asset_id);
    }

    fn get_votes_id(env: Env) -> Address {
        env.storage().get_unchecked(&VOTES).unwrap()
    }

    fn get_native_asset_id(env: Env) -> Address {
        env.storage().get_unchecked(&NATIVE).unwrap()
    }

    fn create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao {

        // Reserve DAO Tokens
        let native_asset_id = env.storage().get_unchecked(&NATIVE).unwrap();
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

        let native_asset_id = env.storage().get_unchecked(&NATIVE).unwrap();
        let native_token = token::Client::new(&env, &native_asset_id);
        let contract = &env.current_contract_address();
        native_token.transfer(&contract, &dao_owner, &RESERVE_AMOUNT);

        let votes_id = Self::get_votes_id(env.clone());
        let votes_contract = votes_contract::Client::new(&env, &votes_id);
        votes_contract.remove_configuration(&dao_id, &dao_owner);

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

    fn get_hookpoint(env: Env, dao_id: Bytes) -> Address {
        if !env.storage().has(&DaoArtifact::Hookpoint(dao_id.clone())) {
            panic_with_error!(env, CoreError::NoHookpoint)
        }
        env.storage()
            .get_unchecked(&DaoArtifact::Hookpoint(dao_id))
            .unwrap()
    }

    fn has_hookpoint(env: Env, dao_id: Bytes) -> bool {
        env.storage().has(&DaoArtifact::Hookpoint(dao_id))
    }

    fn set_hookpoint(env: Env, dao_id: Bytes, hookpoint: Address, dao_owner: Address) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        env.storage().set(&DaoArtifact::Hookpoint(dao.id), &hookpoint);
    }

    fn remove_hookpoint(env: Env, dao_id: Bytes, dao_owner: Address) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        if env.storage().has(&DaoArtifact::Hookpoint(dao_id)) {
            env.storage().remove(&DaoArtifact::Hookpoint(dao.id))
        }
    }
}
