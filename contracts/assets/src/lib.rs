use soroban_sdk::{contractimpl, Env, Bytes, Address, IntoVal};

#[cfg(test)]
mod test;

mod interface;
use interface::AssetTrait;

pub struct AssetContract;


#[contractimpl]
impl AssetTrait for AssetContract {

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
        // todo: implement
        42
    }

    fn authorized(env: Env, id: Address) -> bool {
        // this is always true
        true
    }

    /// Get the allowance for "spender" to transfer from "from".
    fn allowance(env: Env, from: Address, spender: Address, ) -> i128 {
        // todo: implement
        42
    }
   
    fn decimals(env: Env) -> u32 {
        18
    }

    fn name(env: Env) -> Bytes {
        "name".into_val(&env)
        
    }

    fn symbol(env: Env) -> Bytes {
        "symbol".into_val(&env)
    }
    
    fn get_balance_at(env: Env, id: Address, block_number: i128) -> i128 {
        42
    }
    
}