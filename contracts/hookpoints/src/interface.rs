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
    fn on_before_proposal_creation(_env: Env, _dao_id: Bytes, _proposal_owner: Address);
}
