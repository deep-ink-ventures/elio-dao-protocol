use soroban_sdk::{Bytes, Env, Address};

mod hookpoints_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_hookpoints.wasm");
}

use hookpoints_contract::Client as HookpointsContractClient;
use crate::types::DaoArtifact;

fn get_hookpoint(env: &Env, dao_id: &Bytes) -> Option<Address> {
     env.storage()
            .persistent()
            .get(&DaoArtifact::Hookpoint(dao_id.clone()))
         .unwrap_or(None)
}

pub fn on_before_destroy_dao(env: &Env, dao_id: &Bytes) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_destroy_dao(dao_id)
    }
}

pub fn on_before_change_owner(env: &Env, dao_id: &Bytes, new_owner: &Address, dao_owner: &Address) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_change_owner(dao_id, new_owner, dao_owner)
    }
}