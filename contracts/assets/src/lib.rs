use soroban_sdk::{contractimpl, Env, Bytes, Address, IntoVal, Symbol};

#[cfg(test)]
mod test;

mod interface;
use interface::AssetTrait;

mod types;
use types::Token;

pub struct AssetContract;


const TOKEN: Symbol = Symbol::short("TOKEN");

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contractimpl]
impl AssetTrait for AssetContract {

    fn initialize(env: Env, symbol: Bytes, name: Bytes) {
        let token = Token::create(&env, symbol, name);
        env.events().publish((TOKEN, Symbol::short("created")), token.clone());
    }

    fn incr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        let allowance = Token::read_allowance(&env, from.clone(), spender.clone());
        let new_allowance = allowance
            .checked_add(amount)
            .expect("Updated allowance doesn't fit in an i128");

        Token::write_allowance(&env, from.clone(), spender.clone(), new_allowance);
        env.events().publish(
            (Symbol::new(&env, "increase_allowance"), from, spender),
            amount
        );

        // todo: add tests once allowance is fully implemented
    }

    fn decr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        // todo: implement
    }
    fn xfer(env: Env, from: Address, to: Address, amount: i128) {
        // todo: implement
    }

    fn xfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        // todo: implement
    }

    fn balance(env: Env, id: Address) -> i128 {
        // todo: implement
        42
    }

    fn spendable(env: Env, id: Address) -> i128 {
        // just the balance for our purposes
        Self::balance(env, id)
    }

    fn authorized(_env: Env, _id: Address) -> bool {
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
    
    fn get_balance_at(env: Env, id: Address, block_number: i128) -> i128 {
        42
    }
    
}