use soroban_sdk::{contractimpl, Env, Bytes, Address, IntoVal, Symbol};

#[cfg(test)]
mod test;

mod interface;
use interface::AssetTrait;

mod types;
use types::Token;

pub struct AssetContract;


const TOKEN: Symbol = Symbol::short("TOKEN");


#[contractimpl]
impl AssetTrait for AssetContract {

    fn initialize(env: Env, symbol: Bytes, name: Bytes) {
        let token = Token::create(&env, symbol, name);
        env.events().publish((TOKEN, Symbol::short("created")), token.clone());
    }

    fn incr_allow(env: Env, from: Address, spender: Address, amount: i128) {
        // todo: implement
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
        // todo: implement
        42
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