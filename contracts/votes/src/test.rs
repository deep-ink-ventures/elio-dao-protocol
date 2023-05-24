#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, IntoVal,
};

use crate::{
    types::{PropStatus, PROPOSAL_DURATION, PROPOSAL_MAX_NR},
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

#[test]
fn set_metadata() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let owner = Address::random(&client.env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let url = "https://deep-ink.ventures".into_val(&client.env);
    let hash =
        "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&client.env);

    client.set_metadata(&dao_id, &proposal_id, &url, &hash, &owner);

    let meta = client.get_metadata(&proposal_id);
    assert_eq!(meta.url, url);
    assert_eq!(meta.hash, hash);
}

#[test]
#[should_panic(expected = "only the owner can set metadata")]
fn set_metadata_only_owner() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let owner = Address::random(&client.env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let url = "https://deep-ink.ventures".into_val(&client.env);
    let hash =
        "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&client.env);

    let whoever = Address::random(&client.env);
    client.set_metadata(&dao_id, &proposal_id, &url, &hash, &whoever);
}

#[test]
#[should_panic(expected = "metadata does not exist")]
fn non_existing_meta_panics() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let owner = Address::random(&client.env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    client.get_metadata(&proposal_id);
}

#[test]
fn mark_faulty() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);
    let owner = Address::random(&client.env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let reason = "bad".into_val(&client.env);

    client.fault_proposal(&dao_id, &proposal_id, &reason, &owner);

    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.inner.status, PropStatus::Faulty(reason));
}
