use soroban_sdk::{Env, Address, Bytes, Symbol};

pub trait VotesTrait {
    
  fn create_proposal(env: Env, dao_id: Symbol, proposal_id: Symbol, proposal_owner: Address);
  
  fn set_metadata(env: Env, proposal_id: Symbol, meta: Bytes, hash: Bytes, proposal_owner: Address);
  
  fn fault_proposal(env: Env, proposal_id: Symbol, reason: Bytes, dao_owner: Address);
  
  fn finalize_proposal(env: Env, proposal_id: Symbol);
  
  fn vote(env: Env, proposal_id: Symbol, in_favor: bool, voter: Address);
  
}