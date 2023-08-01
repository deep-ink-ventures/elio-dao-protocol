use soroban_sdk::{Address, Env, Bytes};

pub trait HookpointsTrait {

    /// Called before destroying a DAO.
    ///
    /// - `dao_id`: The dao id that will be destroyed.
    fn on_before_destroy_dao(env: Env, dao_id: Bytes);

    /// Called before changing the owner of a DAO.
    ///
    /// - `dao_id`: The dao id that will be destroyed.
    /// - `new_owner`: The address of the new owner.
    /// - `dao_owner`: The address of the original owner.
    fn on_before_change_owner(env: Env, dao_id: Bytes, new_owner: Address, dao_owner: Address);

    /// Called when a vote for a specific user is casted. Should / can return an adjusted voting amount.
    ///
    /// - `dao_id`: The dao id that has been voted for
    /// - `proposal_id`: The proposal id in question
    /// - `account_id`: Address of the voter
    /// - `amount`: The number of tokens at the last checkpoint at or before the vote
    fn on_vote(env: Env, dao_id: Bytes, proposal_id: u32, account_id: Address, amount: i128) -> i128;

    /// Called before proposal creation.
    ///
    /// - `dao_id`: The dao id where proposal was created
    /// - `proposal_owner`: The owner of the proposal
    fn on_before_proposal_creation(env: Env, dao_id: Bytes, proposal_owner: Address);

    /// Called before setting metadata
    ///
    /// - `dao_id`: The dao id where metadata has been set
    /// - `proposal_id`: The id of the proposal
    /// - `meta`: The meta combined with hash to produce metadata
    /// - `hash`: The hash combined with meta to produce metadata
    /// - `proposal_owner`: The owner of the proposal
    fn on_before_set_metadata(env:Env, dao_id: Bytes, proposal_id: u32, meta: Bytes, hash: Bytes, proposal_owner: Address);

    /// Called when a configuration has been set. Should / can return an adjusted proposal_duration.
    ///
    /// - `dao_id`: The dao id that has been configured for.
    /// - `proposal_duration`: The amount of blocks the proposal is active.
    fn on_set_configuration(env: Env, dao_id: Bytes, proposal_duration: u32) -> u32;

    /// Called before declaring proposal faulty.
    ///
    /// - `dao_id`: The dao id that will be declared faulty.
    /// - `proposal_id`: The id of the proposal to be declared faulty.
    /// - `reason`: The reason of declaring the proposal faulty.
    fn on_before_fault_proposal(env: Env, dao_id: Bytes, proposal_id: u32, reason: Bytes);

    /// Called before finalizing a proposal.
    ///
    /// - `dao_id`: The dao id that will be finalized for.
    /// - `proposal_id`: The id of the proposal that will be declared finalized.
    fn on_before_finalize_proposal(env: Env, dao_id: Bytes, proposal_id: u32);

    /// Called before marking the proposal implemented.
    ///
    /// - `dao_id`: The dao id that will be implemented for.
    /// - `proposal_id`: The id of the proposal to be declared implemented.
    fn on_before_mark_implemented(env: Env, dao_id: Bytes, proposal_id: u32);

    /// Called when assets contract increases allowance.
    ///
    /// - `dao_id`: The dao id that will be implemented for.
    /// - `from`: The address requesting the allowance increase. Needs authentication.
    /// - `spender`: The address of the spender.
    /// - `amount`: The amount to increase the allowance.
    fn on_incr_allowance(env: Env, dao_id: Bytes, from: Address, spender: Address, amount: i128) -> i128;

    /// Called when assets contract decrease allowance.
    ///
    /// - `dao_id`: The dao id that will be implemented for.
    /// - `from`: The address requesting the allowance decrease. Needs authentication.
    /// - `spender`: The address of the spender.
    /// - `amount`: The amount to decrease the allowance.
    fn on_decr_allowance(env: Env, dao_id: Bytes, from: Address, spender: Address, amount: i128) -> i128;

    /// Called when assets contract is being transferred.
    ///
    /// - `dao_id`: The dao id that will be implemented for.
    /// - `from`: The address sending the asset.
    /// - `to`: The address receiving the asset.
    /// - `amount`: The amount to be sent.
    fn on_xfer(env: Env, dao_id: Bytes, from: Address, to: Address, amount: i128) -> i128;

    /// Called when assets contract is being transferred for an address.
    ///
    /// - `spender`: The address calling the transaction.
    /// - `from`: The address sending the asset.
    /// - `to`: The address receiving the asset.
    /// - `amount`: The amount to be sent.
    fn on_xfer_from(env: Env, dao_id: Bytes, spender: Address, from: Address, to: Address, amount: i128) -> i128;
}
