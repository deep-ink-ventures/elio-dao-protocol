#![cfg(test)]

use soroban_sdk::{Env, IntoVal};

use crate::{VotesContract, VotesContractClient};

fn create_client() -> VotesContractClient {
    let env = Env::default();
    let contract_id = env.register_contract(None, VotesContract);
    let client = VotesContractClient::new(&env, &contract_id);
    client
}

#[test]
fn active_proposals_are_managed() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let proposal_1_id = "P1".into_val(&client.env);
    let proposal_2_id = "P2".into_val(&client.env);

    client.create_proposal(&dao_id, &proposal_1_id);
    client.create_proposal(&dao_id, &proposal_2_id);

    let all_proposals = client.get_active_proposals();
    assert_eq!(all_proposals.len(), 2);
    let p1 = all_proposals.get_unchecked(0).unwrap();
    let p2 = all_proposals.get_unchecked(1).unwrap();
    assert_eq!(p1.id, proposal_1_id);
    assert_eq!(p2.id, proposal_2_id);

    assert_eq!(p1.ledger, client.env.ledger().sequence());
    assert_eq!(p2.ledger, client.env.ledger().sequence());
}
