#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, IntoVal,
};

use crate::{
    types::{PROPOSAL_DURATION, PROPOSAL_MAX_NR},
    VotesContract, VotesContractClient,
};

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
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
    });
    let owner = Address::random(&client.env);
    let proposal_1_id = client.create_proposal(&dao_id, &owner);

    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 200,
        network_id: Default::default(),
        base_reserve: 10,
    });
    let proposal_2_id = client.create_proposal(&dao_id, &owner);

    let all_proposals = client.get_active_proposals(&dao_id);
    assert_eq!(all_proposals.len(), 2);
    let p1 = all_proposals.get_unchecked(0).unwrap();
    let p2 = all_proposals.get_unchecked(1).unwrap();
    assert_eq!(p1.id, proposal_1_id);
    assert_eq!(p2.id, proposal_2_id);

    assert_eq!(p1.inner.ledger, 100);
    assert_eq!(p2.inner.ledger, 200);

    // outdate the first
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
    });

    let all_proposals = client.get_active_proposals(&dao_id);
    assert_eq!(all_proposals.len(), 1);
    let p = all_proposals.get_unchecked(0).unwrap();
    assert_eq!(p.id, proposal_2_id);
}

#[test]
#[should_panic(expected = "maximum number")]
fn max_number_of_proposals() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let owner = Address::random(&client.env);
    for _ in 0..=PROPOSAL_MAX_NR {
        let _ = client.create_proposal(&dao_id, &owner);
    }
}

#[test]
fn vote() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let owner = Address::random(&client.env);
    let proposal_id = client.create_proposal(&dao_id, &owner);
    let voter = Address::random(&client.env);
    client.vote(&dao_id, &proposal_id, &true, &voter);
    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.in_favor, 1);
}
