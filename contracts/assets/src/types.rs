use soroban_sdk::{contracttype, Bytes, Address, Env, IntoVal};

#[derive(Clone)]
#[contracttype]
pub struct Allowances {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum Token {
    Allowance(Allowances),
    Balance(Address),
    Nonce(Address),
    Name,
    Symbol,
}


impl Token {
    
    pub fn read_allowance(env: &Env, from: Address, spender: Address) -> i128 {
        let key = Self::Allowance(Allowances { from, spender });
        if let Some(allowance) = env.storage().get(&key) {
            allowance.unwrap()
        } else {
            0
        }
    }

    pub fn write_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
        let key = Self::Allowance(Allowances { from, spender });
        env.storage().set(&key, &amount);
    }

    pub fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
        let allowance = Self::read_allowance(env, from.clone(), spender.clone());
        if allowance < amount {
            panic!("insufficient allowance");
        }
        Self::write_allowance(env, from, spender, allowance - amount);
    }
    
    pub fn get_symbol(env: &Env) -> Bytes {
        env.storage().get_unchecked(&Token::Symbol).unwrap()
    }
    
    pub fn get_name(env: &Env) -> Bytes {
        env.storage().get_unchecked(&Token::Name).unwrap()
    }
    
    /// Create a new token
    pub fn create(env: &Env, symbol: Bytes, name: Bytes) {
        if !env.storage().has(&Token::Symbol) {
            env.storage().set(&Token::Symbol, &symbol);
            env.storage().set(&Token::Name, &name);
        } else {
            panic!("DAO already issued a token")
        }
    }
    
    pub fn write_balance(env: &Env, addr: Address, amount: i128) {
        let key = Token::Balance(addr);
        env.storage().set(&key, &amount);
    }
    
    pub fn read_balance(env: &Env, addr: Address) -> i128 {
        let key = Token::Balance(addr);
        if let Some(balance) = env.storage().get(&key) {
            balance.unwrap()
        } else {
            0
        }
    }
}