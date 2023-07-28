#![no_std]
use soroban_sdk::{contractimpl, contract, Address, Env, Bytes};

mod votes_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_votes.wasm");
}

mod assets_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_assets.wasm");
}

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

pub use core_contract::Client as CoreContractClient;
pub use votes_contract::Client as VotesContractClient;
pub use assets_contract::Client as AssetsContractClient;

#[cfg(test)]
mod test;

mod interface;
use interface::HookpointsTrait;

#[contract]
pub struct HookpointsContract;

#[contractimpl]
impl HookpointsTrait for HookpointsContract {
    /* Core HookPoints */
    fn on_before_dao(_env: Env, _dao_id: Bytes) {
        // add functionality here
    }

    fn on_before_change_owner(_env: Env, _dao_id: Bytes) {
        // add functionality here
    }

    /* Votes HookPoints */
    fn on_vote(_env: Env, _dao_id: Bytes, _proposal_id: u32, _account_id: Address, amount: i128) -> i128 {
        amount
    }

    fn on_before_proposal_creation(_env: Env, _dao_id: Bytes, _proposal_owner: Address) {
        // add functionality here
    }

    fn on_before_set_metadata(_env: Env, _dao_id: Bytes, _proposal_id: u32, _meta: Bytes, _hash: Bytes, _proposal_owner: Address) {
        // add functionality here
    }

    fn on_set_configuration(_env: Env, _dao_id: Bytes, proposal_duration: u32) -> u32 {
        proposal_duration
    }

    fn on_before_fault_proposal(_env: Env, _dao_id: Bytes, _proposal_id: u32, _reason: Bytes) {
        // add functionality here
    }

    fn on_before_finalize_proposal(_env: Env, _dao_id: Bytes, _proposal_id: u32) {
        // add functionality here
    }

    fn on_before_mark_implemented(_env: Env, _dao_id: Bytes, _proposal_id: u32) {
        // add functionality here
    }

    /* Assets HookPoints */
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
}
