use soroban_sdk::{Bytes, Env, Address};
use crate::events::CORE;

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

mod hookpoints_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_hookpoints.wasm");
}
use core_contract::Client as CoreContractClient;
use hookpoints_contract::Client as HookpointsContractClient;

fn get_hookpoint(env: &Env, dao_id: &Bytes) -> Option<Address> {
    let core_id = env.storage().instance().get(&CORE).unwrap();
    let core = CoreContractClient::new(env, &core_id);

    if core.has_hookpoint(dao_id) {
        Some(core.get_hookpoint(dao_id))
    } else {
        None
    }
}

pub fn on_vote(env: &Env, dao_id: &Bytes, proposal_id: &u32, account_id: &Address, amount: i128) -> i128 {
    match get_hookpoint(env, dao_id) {
        None => amount,
        Some(addr) => {
            let hookpoints_client = HookpointsContractClient::new(env, &addr);
            hookpoints_client.on_vote(dao_id, proposal_id, account_id, &amount)
        }
    }
}

pub fn on_before_proposal_creation(env: &Env, dao_id: &Bytes, proposal_owner: &Address) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_proposal_creation(dao_id, proposal_owner);
    }
}

pub fn on_before_set_metadata(env: &Env, dao_id: &Bytes, proposal_id: u32, meta: &Bytes, hash: &Bytes, proposal_owner: &Address) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_set_metadata(dao_id, &proposal_id, meta, hash, proposal_owner);
    }
}

pub fn on_set_configuration(_env:&Env, _dao_id: &Bytes, proposal_duration: u32) -> u32 {
    proposal_duration
}

pub fn on_before_fault_proposal(env: &Env, dao_id: &Bytes, proposal_id: u32, reason: &Bytes) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_fault_proposal(dao_id, &proposal_id, reason);
    }
}

pub fn on_before_finalize_proposal(env: &Env, dao_id: &Bytes, proposal_id: u32) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_finalize_proposal(dao_id, &proposal_id);
    }
}

pub fn on_before_mark_implemented(env: &Env, dao_id: &Bytes, proposal_id: u32) {
    if let Some(addr) = get_hookpoint(env, dao_id) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        hookpoints_client.on_before_finalize_proposal(dao_id, &proposal_id);
    }
}