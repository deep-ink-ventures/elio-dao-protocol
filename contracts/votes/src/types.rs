use soroban_sdk::{contracttype, Bytes, Env, Vec, Symbol};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveProposal {
    pub id: Bytes,
    pub ledger: u32,
}

const ACTIVE: Symbol = Symbol::short("ACTIVE");
const PROPOSAL_DURATION: u32 = 10_000;

impl ActiveProposal {
    
    pub fn add(env: &Env, id: Bytes) {
        let mut proposals = ActiveProposal::get_all(env);

        if proposals.len() == 25 {
            panic!("max number of proposals reached")
        }
        proposals.push_back(ActiveProposal {id, ledger: env.ledger().sequence()});
        env.storage().set(&ACTIVE, &proposals);
    }
    
    pub fn get_all(env: &Env) -> Vec<ActiveProposal> {
        if !env.storage().has(&ACTIVE) {
            return Vec::new(env)
        }
        env.storage().get_unchecked(&ACTIVE).unwrap()
    }        
}

