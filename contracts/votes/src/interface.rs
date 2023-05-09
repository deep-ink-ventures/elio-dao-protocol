use soroban_sdk::{Env, Address, Bytes, Vec};

use crate::types::ActiveProposal;

pub trait VotesTrait {
    
  fn create_proposal(env: Env, dao_id: Bytes, proposal_id: Bytes);
  
  fn set_metadata(env: Env, proposal_id: Bytes, meta: Bytes, hash: Bytes, proposal_owner: Address);
  
  fn fault_proposal(env: Env, proposal_id: Bytes, reason: Bytes, dao_owner: Address);
  
  fn finalize_proposal(env: Env, proposal_id: Bytes);
  
  fn vote(env: Env, proposal_id: Bytes, in_favor: bool, voter: Address);
  
  fn get_active_proposals(env: Env) -> Vec<ActiveProposal>;
  
}