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

    fn on_vote(_env: Env, _dao_id: Bytes, _proposal_id: u32, _account_id: Address, amount: i128) -> i128 {
        amount
    }
}
