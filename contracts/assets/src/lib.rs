#![no_std]

use events::{
    AssetMintedEventData, AssetNewOwnerEventData, AssetSetGovernanceIDEventData,
    AssetTransferredEventData, ASSET, CORE_ADDRESS_CHANGED, MINTED, OWNER_CHANGED, TRANSFERRED,
};
use soroban_sdk::{contractimpl, contract, Address, Bytes, Env, Symbol, panic_with_error};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

mod votes_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_votes.wasm");
}

#[cfg(test)]
mod test;

mod events;

mod interface;
use interface::AssetTrait;

mod types;
mod error;

use types::{Checkpoint, Token};
use crate::error::AssetError;

#[contract]
pub struct AssetContract;

fn check_non_negative_amount(env: &Env, amount: i128) {
    if amount < 0 {
        panic_with_error!(env, AssetError::NegativeAmount)
    }
}

#[contractimpl]
impl AssetTrait for AssetContract {
    fn init(env: Env, symbol: Bytes, name: Bytes, owner: Address, core_address: Address) {
        Token::create(&env, &symbol, &name, &owner, &core_address);
    }

    fn mint(env: Env, owner: Address, supply: i128) {
        Token::check_auth(&env, &owner);
        Token::check_is_minted(&env, owner.clone());
        Token::write_balance(&env, owner.clone(), supply);
        env.events().publish(
            (ASSET, MINTED, Token::get_symbol(&env)),
            AssetMintedEventData {
                owner_id: owner,
                amount: supply,
            },
        );
    }

    fn set_owner(env: Env, owner: Address, new_owner: Address) {
        Token::set_owner(&env, &owner, &new_owner);
        env.events().publish(
            (ASSET, OWNER_CHANGED, Token::get_symbol(&env)),
            AssetNewOwnerEventData {
                new_owner_id: new_owner,
            },
        );
    }

    fn owner(env: Env) -> Address {
        Token::get_owner(&env)
    }

    fn set_core_address(env: Env, owner: Address, core_address: Address) {
        Token::set_core_address(&env, &owner, &core_address);
        env.events().publish(
            (ASSET, CORE_ADDRESS_CHANGED, Token::get_symbol(&env)),
            AssetSetGovernanceIDEventData { core_address },
        );
    }

    fn core_address(env: Env) -> Address {
        Token::get_core_address(&env)
    }

    fn incr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        check_non_negative_amount(&env, amount);
        let allowance = Token::read_allowance(&env, from.clone(), spender.clone());
        let new_allowance = allowance + amount;

        Token::write_allowance(&env, from.clone(), spender.clone(), new_allowance);
        env.events().publish(
            (Symbol::new(&env, "increase_allowance"), from, spender),
            amount,
        );
    }

    fn decr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        check_non_negative_amount(&env, amount);

        let allowance = Token::read_allowance(&env, from.clone(), spender.clone());
        if amount >= allowance {
            Token::write_allowance(&env, from.clone(), spender.clone(), 0);
        } else {
            Token::write_allowance(&env, from.clone(), spender.clone(), allowance - amount);
        }
        env.events().publish(
            (Symbol::new(&env, "decrease_allowance"), from, spender),
            amount,
        );
    }

    fn xfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_non_negative_amount(&env, amount);
        Token::spend_balance(&env, from.clone(), amount);
        Token::receive_balance(&env, to.clone(), amount);
        env.events().publish(
            (ASSET, TRANSFERRED, Token::get_symbol(&env)),
            AssetTransferredEventData {
                owner_id: from,
                new_owner_id: to,
                amount,
            },
        );
    }

    fn xfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_non_negative_amount(&env, amount);
        Token::spend_allowance(&env, from.clone(), spender, amount);
        Token::spend_balance(&env, from.clone(), amount);
        Token::receive_balance(&env, to.clone(), amount);
        env.events().publish(
            (ASSET, TRANSFERRED, Token::get_symbol(&env)),
            AssetTransferredEventData {
                owner_id: from,
                new_owner_id: to,
                amount,
            },
        );
    }

    fn balance(env: Env, addr: Address) -> i128 {
        Token::read_balance(&env, addr)
    }

    fn spendable(env: Env, addr: Address) -> i128 {
        // just the balance for our purposes
        Self::balance(env, addr)
    }

    fn authorized(_env: Env, _addr: Address) -> bool {
        // this is always true
        true
    }

    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        Token::read_allowance(&env, from, spender)
    }

    fn decimals(_env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> Bytes {
        Token::get_name(&env)
    }

    fn symbol(env: Env) -> Bytes {
        Token::get_symbol(&env)
    }

    fn get_checkpoint_count(env: Env, id: Address) -> u32 {
        Token::get_checkpoints(&env, id).len()
    }

    fn get_checkpoint_at(env: Env, id: Address, i: u32) -> Checkpoint {
        Token::get_checkpoint_at(&env, id, i)
    }

    fn get_balance_at(env: Env, addr: Address, sequence: u32) -> i128 {
        let checkpoint = Token::get_checkpoint_for_sequence(&env, addr, sequence);
        match checkpoint {
            Some(cp) => cp.balance,
            None => 0
        }
    }
}
