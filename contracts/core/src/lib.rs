#![no_std]

//use soroban_sdk::xdr::{Asset, Hash, HashIdPreimage, HashIdPreimageFromAsset, WriteXdr};
use soroban_sdk::{contractimpl, log, token, Address, Bytes, BytesN, Env, Symbol};

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

pub const NATIVE: Symbol = Symbol::short("NATIVE");

pub struct CoreContract;

const RESERVE_AMOUNT: i128 = 1;

#[contractimpl]
impl CoreTrait for CoreContract {
    fn init(env: Env, votes_id: Address, native_asset_id: Address) {
        if env.storage().has(&VOTES) {
            panic!("Already initialized")
        }

        env.storage().set(&VOTES, &votes_id);
        env.storage().set(&NATIVE, &native_asset_id);
        //.unwrap_or_else(native_asset_contract_address));

        //.unwrap_or_else(|| {
        //     let preimage_from_asset = HashIdPreimageFromAsset {
        //         network_id: Hash(env.ledger().network_id().into()),
        //         asset: Asset::Native,
        //     };
        //     let preimage = HashIdPreimage::ContractIdFromAsset(preimage_from_asset);
        //     let bytes = Bytes::from_slice(&env, &preimage.to_xdr().unwrap());
        //     Address::from_contract_id(&env.crypto().sha256(&bytes))
        // }));
    }

    fn get_votes_id(env: Env) -> Address {
        env.storage().get_unchecked(&VOTES).unwrap()
    }

    fn create_dao(env: Env, dao_id: Bytes, dao_name: Bytes, dao_owner: Address) -> Dao {
        log!(&env, "reserving native tokens");
        let native_asset_id = env.storage().get_unchecked(&NATIVE).unwrap();
        let native_token = token::Client::new(&env, &native_asset_id);
        let contract = &env.current_contract_address();
        native_token.transfer(&dao_owner, &contract, &RESERVE_AMOUNT);

        log!(&env, "creating DAO");
        let dao = Dao::create(&env, dao_id.clone(), dao_name.clone(), dao_owner.clone());

        log!(&env, "publishing DAO CREATED event");
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

        log!(&env, "releasing native token reserve");
        let native_asset_id = env.storage().get_unchecked(&NATIVE).unwrap();
        let native_token = token::Client::new(&env, &native_asset_id);
        let contract = &env.current_contract_address();
        native_token.transfer(&contract, &dao_owner, &RESERVE_AMOUNT);

        log!(&env, "publishing DAO DESTROYED event");
        env.events()
            .publish((DAO, DESTROYED), DaoDestroyedEventData { dao_id });
    }

    fn issue_token(
        env: Env,
        dao_id: Bytes,
        dao_owner: Address,
        assets_wasm_hash: BytesN<32>,
        asset_salt: BytesN<32>,
    ) {
        let dao = Dao::load_for_owner(&env, &dao_id, &dao_owner);
        log!(env, "issuing DAO token");
        dao.issue_token(&env, assets_wasm_hash, asset_salt);
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
}

// fn native_asset_contract_address() -> Address {
//     let env = Env::default();
//     let preimage_from_asset = HashIdPreimageFromAsset {
//         network_id: Hash(env.ledger().network_id().into()),
//         asset: Asset::Native,
//     };
//     let preimage = HashIdPreimage::ContractIdFromAsset(preimage_from_asset);
//     let bytes = Bytes::from_slice(&env, &preimage.to_xdr().unwrap());
//     Address::from_contract_id(&env.crypto().sha256(&bytes))
// }
