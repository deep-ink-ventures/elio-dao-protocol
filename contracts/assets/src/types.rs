use soroban_sdk::{contracttype, Address, Bytes, BytesN, Env, Vec};

use crate::{core_contract, votes_contract};

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
    GovernanceId,
    Checkpoints(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub ledger: u32,
    pub balance: i128,
}

impl Token {
    pub fn get_checkpoints(env: &Env, id: Address) -> Vec<Checkpoint> {
        let key = Token::Checkpoints(id);
        if !env.storage().has(&key) {
            return Vec::new(env);
        }
        env.storage().get_unchecked(&key).unwrap()
    }

    pub fn get_checkpoint_at(env: &Env, id: Address, i: u32) -> Checkpoint {
        let checkpoints = Token::get_checkpoints(env, id);
        if checkpoints.len() <= i {
            panic!("Checkpoint index error");
        }
        checkpoints.get_unchecked(i).unwrap()
    }

    /// Returns the closest checkpoint at or BEFORE a given sequence
    pub fn get_checkpoint_for_sequence(env: &Env, id: Address, sequence: u32) -> Checkpoint {
        let checkpoints = Token::get_checkpoints(env, id);
        let mut cp_candidate = checkpoints.first_unchecked().unwrap();

        for checkpoint in checkpoints.iter_unchecked() {
            if checkpoint.ledger > sequence {
                break;
            }
            if checkpoint.ledger > cp_candidate.ledger {
                cp_candidate = checkpoint;
            }
        }
        cp_candidate
    }

    /// Writes a checkpoint for a given balance at the current sequence number
    ///
    /// This prevents double counting (e.g. you vote, sell your tokens and vote again) without
    /// the requirement for staking the tokens during a proposal.
    ///
    /// If you roll your own implementation you need to have a strategy to keep the number of
    /// checkpoints bounded, otherwise the required storage will escalate on busy tokens.
    ///
    /// Our strategy is to set the maximum number of concurrently active proposals to 25;
    /// too many active proposals creates voter fatigue anyway.
    pub fn write_checkpoint(env: &Env, id: Address) {
        let key = Self::Checkpoints(id.clone());

        let governance_id = Token::get_governance_id(env);
        let core_contract = core_contract::Client::new(env, &governance_id);
        let vote_id = core_contract.get_votes_id();
        let votes_contract = votes_contract::Client::new(env, &vote_id);

        let active_proposals = votes_contract.get_active_proposals(&Token::get_symbol(env));

        let mut filtered_checkpoints: Vec<Checkpoint> = Vec::new(env);

        for proposal in active_proposals.iter_unchecked() {
            filtered_checkpoints.push_back(Token::get_checkpoint_for_sequence(
                env,
                id.clone(),
                proposal.inner.ledger,
            ));
        }

        filtered_checkpoints.push_back(Checkpoint {
            balance: Token::read_balance(env, id),
            ledger: env.ledger().sequence(),
        });
        env.storage().set(&key, &filtered_checkpoints);
    }

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
    pub fn create(
        env: &Env,
        symbol: &Bytes,
        name: &Bytes,
        owner: &Address,
        governance_id: &BytesN<32>,
    ) {
        if env.storage().has(&Token::Symbol) {
            panic!("DAO already issued a token")
        }
        env.storage().set(&Token::Symbol, symbol);
        env.storage().set(&Token::Name, name);
        env.storage().set(&Token::Owner, owner);
        env.storage().set(&Token::GovernanceId, governance_id);
    }

    pub fn set_owner(env: &Env, owner: &Address, new_owner: &Address) {
        Token::check_auth(env, owner);
        env.storage().set(&Token::Owner, &new_owner);
    }

    pub fn set_governance_id(env: &Env, owner: &Address, governance_id: &BytesN<32>) {
        Token::check_auth(env, owner);
        env.storage().set(&Token::GovernanceId, governance_id);
    }

    pub fn write_balance(env: &Env, addr: Address, amount: i128) {
        let key = Token::Balance(addr.clone());
        env.storage().set(&key, &amount);
        Token::write_checkpoint(env, addr);
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

    pub fn spend_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Token::read_balance(env, addr.clone());
        if balance < amount {
            panic!("insufficient balance")
        }
        Token::write_balance(env, addr, balance - amount);
    }

    pub fn receive_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Token::read_balance(env, addr.clone());
        Token::write_balance(env, addr, balance + amount);
    }
}
