use soroban_sdk::{contracttype, panic_with_error, Address, Bytes, Env, Vec};

use crate::error::AssetError;
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
    Name,
    Symbol,
    Owner,
    CoreAddress,
    Checkpoints(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Checkpoint {
    pub ledger: u32,
    pub balance: i128,
}

pub const A_WEEK_IN_LEDGERS: u32 = 100800;
pub const BUMP_A_MONTH: u32 = 432000;
pub const BUMP_A_MONTH_THRESHOLD: u32 = 432000 - A_WEEK_IN_LEDGERS;

impl Token {
    pub fn get_checkpoints(env: &Env, id: Address) -> Vec<Checkpoint> {
        let key = Token::Checkpoints(id);
        if !env.storage().persistent().has(&key) {
            return Vec::new(env);
        }
        env.storage()
            .persistent()
            .bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        env.storage().persistent().get(&key).unwrap()
    }

    pub fn get_checkpoint_at(env: &Env, id: Address, i: u32) -> Checkpoint {
        let checkpoints = Self::get_checkpoints(env, id);
        if checkpoints.is_empty() {
            panic_with_error!(env, AssetError::NoCheckpoint)
        }
        if checkpoints.len() <= i {
            panic_with_error!(env, AssetError::CheckpointIndexError)
        }
        checkpoints.get_unchecked(i)
    }

    /// Returns the closest checkpoint at or BEFORE a given sequence
    pub fn get_checkpoint_for_sequence(
        env: &Env,
        id: Address,
        sequence: u32,
    ) -> Option<Checkpoint> {
        let checkpoints = Token::get_checkpoints(env, id);
        let cp_candidate = checkpoints.first();

        cp_candidate.as_ref()?;

        let mut cp = cp_candidate.unwrap();
        for checkpoint in checkpoints.into_iter() {
            if checkpoint.ledger > sequence {
                break;
            }
            if checkpoint.ledger > cp.ledger {
                cp = checkpoint
            }
        }
        Some(cp)
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

        let core_address = Self::get_core_address(env);

        let core_contract = core_contract::Client::new(env, &core_address);
        let vote_id = core_contract.get_votes_id();
        let votes_contract = votes_contract::Client::new(env, &vote_id);

        let active_proposals = votes_contract.get_active_proposals(&Self::get_symbol(env));

        let mut filtered_checkpoints: Vec<Checkpoint> = Vec::new(env);
        for proposal in active_proposals.into_iter() {
            let checkpoint =
                Self::get_checkpoint_for_sequence(env, id.clone(), proposal.inner.ledger);

            if let Some(cp) = checkpoint {
                filtered_checkpoints.push_back(cp);
            }
        }

        filtered_checkpoints.push_back(Checkpoint {
            balance: Token::read_balance(env, id),
            ledger: env.ledger().sequence(),
        });
        env.storage().persistent().set(&key, &filtered_checkpoints);
        env.storage()
            .persistent()
            .bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
    }

    pub fn read_allowance(env: &Env, from: Address, spender: Address) -> i128 {
        let key = Self::Allowance(Allowances { from, spender });

        let allowance = env.storage().persistent().get(&key).unwrap_or(0);
        if allowance > 0 {
            env.storage()
                .persistent()
                .bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        }
        allowance
    }

    pub fn write_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
        let key = Self::Allowance(Allowances { from, spender });
        env.storage().persistent().set(&key, &amount);
        env.storage()
            .persistent()
            .bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
    }

    pub fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
        let allowance = Self::read_allowance(env, from.clone(), spender.clone());
        if allowance < amount {
            panic_with_error!(env, AssetError::InsufficientAllowance)
        }
        Self::write_allowance(env, from, spender, allowance - amount);
    }

    pub fn get_symbol(env: &Env) -> Bytes {
        env.storage().instance().get(&Token::Symbol).unwrap()
    }

    pub fn get_name(env: &Env) -> Bytes {
        env.storage().instance().get(&Token::Name).unwrap()
    }

    pub fn get_owner(env: &Env) -> Address {
        env.storage().instance().get(&Token::Owner).unwrap()
    }

    pub fn get_core_address(env: &Env) -> Address {
        env.storage().instance().get(&Token::CoreAddress).unwrap()
    }

    /// Create a new token
    pub fn create(
        env: &Env,
        symbol: &Bytes,
        name: &Bytes,
        owner: &Address,
        core_address: &Address,
    ) {
        if env.storage().instance().has(&Token::Symbol) {
            panic_with_error!(env, AssetError::DaoAlreadyIssuedToken)
        }
        env.storage().instance().set(&Token::Symbol, symbol);
        env.storage().instance().set(&Token::Name, name);
        env.storage().instance().set(&Token::Owner, owner);
        env.storage()
            .instance()
            .set(&Token::CoreAddress, core_address);

        env.storage()
            .instance()
            .bump(BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
    }

    pub fn set_owner(env: &Env, owner: &Address, new_owner: &Address) {
        Token::check_auth(env, owner);
        env.storage().instance().set(&Token::Owner, &new_owner);
    }

    pub fn set_core_address(env: &Env, owner: &Address, core_address: &Address) {
        Token::check_auth(env, owner);
        env.storage()
            .instance()
            .set(&Token::CoreAddress, core_address);
    }

    pub fn write_balance(env: &Env, addr: Address, amount: i128) {
        let key = Token::Balance(addr.clone());
        env.storage().persistent().set(&key, &amount);
        env.storage()
            .persistent()
            .bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        Token::write_checkpoint(env, addr);
    }

    pub fn read_balance(env: &Env, addr: Address) -> i128 {
        let key = Token::Balance(addr);
        let balance = env.storage().persistent().get(&key).unwrap_or(0);
        if balance > 0 {
            env.storage()
                .persistent()
                .bump(&key, BUMP_A_MONTH_THRESHOLD, BUMP_A_MONTH);
        }
        balance
    }

    pub fn check_auth(env: &Env, owner: &Address) {
        owner.require_auth();
        if owner != &Token::get_owner(env) {
            panic_with_error!(env, AssetError::NotTokenOwner)
        }
    }

    pub fn check_is_minted(env: &Env, owner: Address) {
        if !Token::get_checkpoints(env, owner).is_empty() {
            panic_with_error!(env, AssetError::CanOnlyBeMintedOnce)
        }
    }

    pub fn spend_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Token::read_balance(env, addr.clone());
        if balance < amount {
            panic_with_error!(env, AssetError::InsufficientBalance)
        }
        Token::write_balance(env, addr, balance - amount);
    }

    pub fn receive_balance(env: &Env, addr: Address, amount: i128) {
        let balance = Token::read_balance(env, addr.clone());
        Token::write_balance(env, addr, balance + amount);
    }
}
