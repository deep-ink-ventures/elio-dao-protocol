use soroban_sdk::{Address, Env, Bytes};

pub trait HookpointsTrait {

    /// Called when a vote for a specific user is casted. Should / can return an adjusted voting amount.
    ///
    /// - `dao_id`: The dao id that has been voted for
    /// - `proposal_id`: The proposal id in question
    /// - `account_id`: Address of the voter
    /// - `amount`: The number of tokens at the last checkpoint at or before the vote
    fn on_vote(env: Env, dao_id: Bytes, proposal_id: u32, account_id: Address, amount: i128) -> i128;

    /// Called when a vote for a specific user is casted. Should / can return an adjusted voting amount.
    ///
    /// - `dao_id`: The dao id that has been voted for
    /// - `proposal_owner`: The owner of the proposal
    fn on_before_proposal_creation(env: Env, dao_id: Bytes, proposal_owner: Address);
    fn on_before_set_metadata(env:Env, dao_id: Bytes, proposal_id: u32, meta: Bytes, hash: Bytes, proposal_owner: Address);
    fn on_set_configuration(env: Env, dao_id: Bytes, proposal_duration: u32) -> u32;
    fn on_before_fault_proposal(env: Env, dao_id: Bytes, proposal_id: u32, reason: Bytes);
    fn on_before_finalize_proposal(env: Env, dao_id: Bytes, proposal_id: u32);
    fn on_before_mark_implemented(env: Env, dao_id: Bytes, proposal_id: u32);
    fn on_before_destroy_dao(env: Env, dao_id: Bytes);
    fn on_before_change_owner(env: Env, dao_id: Bytes);
    fn on_incr_allowance(env: Env, from: Address, spender: Address, amount: i128) -> i128;
    fn on_decr_allowance(env: Env, from: Address, spender: Address, amount: i128) -> i128;
    fn on_xfer(env: Env, from: Address, to: Address, amount: i128) -> i128;
    fn on_xfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) -> i128;
}
