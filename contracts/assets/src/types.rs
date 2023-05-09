use soroban_sdk::{contracttype, Bytes, Address, Env, IntoVal, BytesN};

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
    Owner,
    GovernanceId
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
    
    pub fn get_owner(env: &Env) -> Address {
        env.storage().get_unchecked(&Token::Owner).unwrap()
    }

    pub fn get_governance_id(env: &Env) -> BytesN<32> {
        env.storage().get_unchecked(&Token::GovernanceId).unwrap()
    }

    /// Create a new token
    pub fn create(env: &Env, symbol: &Bytes, name: &Bytes, owner: &Address, governance_id: &BytesN<32>) {
        if !env.storage().has(&Token::Symbol) {
            env.storage().set(&Token::Symbol, symbol);
            env.storage().set(&Token::Name, name);
            env.storage().set(&Token::Owner, owner);
            env.storage().set(&Token::GovernanceId, governance_id);
        } else {
            panic!("DAO already issued a token")
        }
    }
    
    pub fn set_owner(env: &Env, owner: &Address, new_owner: &Address) {
        Token::check_auth(env, owner);
        if owner != &Token::get_owner(env) {
            panic!("not Token owner")
        }
        env.storage().set(&Token::Owner, &new_owner);
    }

    pub fn set_governance_id(env: &Env, owner: &Address, governance_id: &BytesN<32>) {
        Token::check_auth(env, owner);
        env.storage().set(&Token::GovernanceId, governance_id);
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

    pub fn check_auth(env: &Env, owner: &Address) {
        owner.require_auth();
        if owner != &Token::get_owner(env) {
            panic!("not Token owner")
        }
    }
}