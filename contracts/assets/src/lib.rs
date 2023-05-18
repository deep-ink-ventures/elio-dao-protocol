#![no_std]

use soroban_sdk::{contractimpl, Address, Bytes, BytesN, Env, Symbol};

mod core_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_core.wasm");
}

mod votes_contract {
    type ProposalId = u32;
    soroban_sdk::contractimport!(file = "../../wasm/elio_votes.wasm");
}

#[cfg(test)]
mod test;

mod interface;
use interface::AssetTrait;

mod types;
use types::{Checkpoint, Token};

pub struct AssetContract;

fn check_non_negative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contractimpl]
impl AssetTrait for AssetContract {
    fn init(
        env: Env,
        symbol: Bytes,
        name: Bytes,
        initial_supply: i128,
        owner: Address,
        governance_id: BytesN<32>,
    ) {
        Token::create(&env, &symbol, &name, &owner, &governance_id);
        Token::write_balance(&env, owner.clone(), initial_supply);
        env.events()
            .publish((Symbol::short("created"), owner, symbol), initial_supply);
    }

    fn set_owner(env: Env, owner: Address, new_owner: Address) {
        Token::set_owner(&env, &owner, &new_owner);
    }

    fn owner(env: Env) -> Address {
        Token::get_owner(&env)
    }

    fn set_governance_id(env: Env, owner: Address, governance_id: BytesN<32>) {
        Token::set_governance_id(&env, &owner, &governance_id);
    }

    fn governance_id(env: Env) -> BytesN<32> {
        Token::get_governance_id(&env)
    }

    fn incr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        check_non_negative_amount(amount);
        let allowance = Token::read_allowance(&env, from.clone(), spender.clone());
        let new_allowance = allowance
            .checked_add(amount.into())
            .expect("Updated allowance doesn't fit in an i128");

        Token::write_allowance(&env, from.clone(), spender.clone(), new_allowance);
        env.events().publish(
            (Symbol::new(&env, "increase_allowance"), from, spender),
            amount,
        );
    }

    fn decr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        check_non_negative_amount(amount);

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

        check_non_negative_amount(amount);
        Token::spend_balance(&env, from.clone(), amount);
        Token::receive_balance(&env, to.clone(), amount);
        env.events()
            .publish((Symbol::new(&env, "transfer"), from, to), amount)
    }

    fn xfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_non_negative_amount(amount);
        Token::spend_allowance(&env, from.clone(), spender, amount);
        Token::spend_balance(&env, from.clone(), amount);
        Token::receive_balance(&env, to.clone(), amount);
        env.events()
            .publish((Symbol::new(&env, "transfer"), from, to), amount)
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
        Token::get_checkpoint_for_sequence(&env, addr, sequence).balance
    }
}
