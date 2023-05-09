use soroban_sdk::{Env, Address, Bytes, BytesN};

/// This follows the official specs w/o admin functionalities.
pub trait AssetTrait {

    /// Initializes the contract
    ///
    /// - `symbol`: The DAO ID
    /// - `name`: Name of the DAO
    /// - `initial_supply`: Total tokens minted on launch
    /// - `governance_id`: Contract ID of the governance protocol to use. We'd be thrilled if you choose elio DAO's latest :-)
    ///
    fn init(env: Env, symbol: Bytes, name: Bytes, initial_supply: i128, owner: Address, governance_id: BytesN<32>);

    // --------------------------------------------------------------------------------
    /// Admin functions
    // --------------------------------------------------------------------------------

    /// Change the owner of this token
    ///
    /// - `owner`: The current owner (must be authed and the current owner, obviously)
    /// - `new_owner`: The new owner
    ///
    fn set_owner(env: Env, owner: Address, new_owner: Address);

    /// Returns the current owner.
    fn owner(env: Env) -> Address;

    /// Change the governance id of this token to either a different implementation or to upgrade to
    /// a newer version of elio DAO.
    ///
    /// - `owner`: The current owner (must be authed and the current owner, obviously)
    /// - `governance_id`: Contract ID of the governance protocol to use. We'd be thrilled if you choose elio DAO's latest :-)
    ///
    fn set_governance_id(env: Env, owner: Address, governance_id: BytesN<32>);

    /// Returns the current governance id.
    fn governance_id(env: Env) -> BytesN<32>;
  
    // --------------------------------------------------------------------------------
    // Token interface
    // --------------------------------------------------------------------------------
    // 
    // All the functions here have to be authorized by the token spender 
    // (usually named `from` here) using all the input arguments, i.e. they have
    // to call  `from.require_auth()`.

    /// Increase the allowance by "amount" for "spender" to transfer/burn from "from".
    /// Emit event with topics = ["incr_allow", from: Address, spender: Address], data = [amount: i128]
    fn incr_allow(env: Env, from: Address, spender: Address, amount: i128);

    /// Decrease the allowance by "amount" for "spender" to transfer/burn from "from".
    /// If "amount" is greater than the current allowance, set the allowance to 0.
    /// Emit event with topics = ["decr_allow", from: Address, spender: Address], data = [amount: i128]
    fn decr_allow(env: Env, from: Address, spender: Address, amount: i128);

    /// Transfer "amount" from "from" to "to.
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [amount: i128]
    fn xfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer "amount" from "from" to "to", consuming the allowance of "spender".
    /// Authorized by spender (`spender.require_auth()`).
    /// Emit event with topics = ["transfer", from: Address, to: Address], data = [amount: i128]
    fn xfer_from( env: Env, spender: Address, from: Address, to: Address, amount: i128);
   
    // --------------------------------------------------------------------------------
    // Read-only Token interface
    // --------------------------------------------------------------------------------
    // 
    // The functions here don't need any authorization and don't emit any
    // events.

    /// Get the balance of "id".
    fn balance(env: Env, id: Address) -> i128;

    /// Get the spendable balance of "id". This will return the same value as balance()
    /// unless this is called on the Stellar Asset Contract, in which case this can
    /// be less due to reserves/liabilities.
    fn spendable(env: Env, id: Address) -> i128;

    // DAO tokens are always authorized, so this is just returning true for our purposes.
    fn authorized(env: Env, id: Address) -> bool;

    /// Get the allowance for "spender" to transfer from "from".
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    // --------------------------------------------------------------------------------
    // Descriptive Interface
    // --------------------------------------------------------------------------------

    // DAO tokens are fixed with 18 decimals.
    fn decimals(env: Env) -> u32;

    // Get the name for this token.
    fn name(env: Env) -> Bytes;

    // Get the symbol for this token.
    fn symbol(env: Env) -> Bytes;
    
    // Get the last recorded historical balance at or before the given block number
    fn get_balance_at(env: Env, id: Address, block_number: i128) -> i128;
}