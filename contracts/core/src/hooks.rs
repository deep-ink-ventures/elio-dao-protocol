use soroban_sdk::{Bytes, Env, Address};

mod hookpoints_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_hookpoints.wasm");
}

use hookpoints_contract::Client as HookpointsContractClient;
use crate::{CoreContractClient};

fn get_hookpoint(env: &Env, dao_id: &Bytes) -> Option<Address> {
    let core = CoreContractClient::new(env, &env.current_contract_address());

    if core.has_hookpoint(dao_id) {
        Some(core.get_hookpoint(dao_id))
    } else {
        None
    }
}

pub fn on_before_destroy_dao(env: &Env, dao_id: &Bytes) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_destroy_dao(dao_id)
    }
}

pub fn on_before_change_owner(env: &Env, dao_id: &Bytes) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_change_owner(dao_id)
    }
}