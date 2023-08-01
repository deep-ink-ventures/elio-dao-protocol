use soroban_sdk::{Env, Address};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

mod hookpoints_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_hookpoints.wasm");
}
use core_contract::Client as CoreContractClient;
use hookpoints_contract::Client as HookpointsContractClient;
use crate::types::Token;

fn get_hookpoint(env: &Env) -> Option<Address> {
    let dao_id = Token::get_symbol(env);
    let core = CoreContractClient::new(env, &Token::get_core_address(env));

    if core.has_hookpoint(&dao_id) {
        Some(core.get_hookpoint(&dao_id))
    } else {
        None
    }
}

pub fn on_incr_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) -> i128 {
    if let Some(addr) = get_hookpoint(env) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        return hookpoints_client.on_incr_allowance(from, spender, &amount);
    }
    amount
}

pub fn on_decr_allowance(env: &Env, from: &Address, spender: &Address, amount: i128) -> i128 {
    if let Some(addr) = get_hookpoint(env) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        return hookpoints_client.on_decr_allowance(from, spender, &amount);
    }
    amount
}

pub fn on_xfer(env: &Env, from: &Address, to: &Address, amount: i128) -> i128 {
    if let Some(addr) = get_hookpoint(env) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        return hookpoints_client.on_xfer(from, to, &amount);
    }
    amount
}

pub fn on_xfer_from(env: &Env, spender: &Address, from: &Address, to: &Address, amount: i128) -> i128 {
    if let Some(addr) = get_hookpoint(env) {
        let hookpoints_client = HookpointsContractClient::new(env, &addr);
        return hookpoints_client.on_xfer_from(spender, from, to, &amount);
    }
    amount
}