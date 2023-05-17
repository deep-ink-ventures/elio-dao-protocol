use soroban_sdk::{Env, Address, Bytes, Vec};

use crate::types::{ActiveProposal, ProposalId};

pub trait VotesTrait {
    
  fn create_proposal(env: Env, dao_id: Bytes, proposal_owner: Address) -> ProposalId;
  
  fn set_metadata(env: Env, proposal_id: ProposalId, meta: Bytes, hash: Bytes, proposal_owner: Address);
  
  fn fault_proposal(env: Env, proposal_id: ProposalId, reason: Bytes, dao_owner: Address);
  
  fn finalize_proposal(env: Env, proposal_id: ProposalId);
  
  fn vote(env: Env, dao_id: Bytes, proposal_id: ProposalId, in_favor: bool, voter: Address);
  
  fn get_active_proposals(env: Env, dao_id: Bytes) -> Vec<ActiveProposal>;
  
}
