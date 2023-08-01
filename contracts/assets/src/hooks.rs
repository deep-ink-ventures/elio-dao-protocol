use soroban_sdk::{Bytes, Env, Address};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

mod hookpoints_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_hookpoints.wasm");
}
use core_contract::Client as CoreContractClient;

fn get_hookpoint(env: &Env, dao_id: &Bytes) -> Option<Address> {
    let core = CoreContractClient::new(env, &env.current_contract_address());

    if core.has_hookpoint(dao_id) {
        Some(core.get_hookpoint(dao_id))
    } else {
        None
    }
}

// todo: hookpoint implementation
fn on_incr_allowance(_env: Env, _from: Address, _spender: Address, amount: i128) -> i128 {
    amount
}

fn on_decr_allowance(_env: Env, _from: Address, _spender: Address, amount: i128) -> i128 {
    amount
}

fn on_xfer(_env: Env, _from: Address, _to: Address, amount: i128) -> i128 {
    amount
}

fn on_xfer_from(_env: Env, _spender: Address, _from: Address, _to: Address, amount: i128) -> i128 {
    amount
}