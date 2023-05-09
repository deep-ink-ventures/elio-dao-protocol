#![cfg(test)]

use soroban_sdk::{Env, IntoVal, testutils::{Ledger, LedgerInfo}};

use crate::{VotesContract, VotesContractClient, types::PROPOSAL_DURATION};

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
    let proposal_1_id = "P1".into_val(&client.env);
    client.create_proposal(&dao_id, &proposal_1_id);
    
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 200,
        network_id: Default::default(),
        base_reserve: 10,
    });
    let proposal_2_id = "P2".into_val(&client.env);

    client.create_proposal(&dao_id, &proposal_2_id);

    let all_proposals = client.get_active_proposals();
    assert_eq!(all_proposals.len(), 2);
    let p1 = all_proposals.get_unchecked(0).unwrap();
    let p2 = all_proposals.get_unchecked(1).unwrap();
    assert_eq!(p1.id, proposal_1_id);
    assert_eq!(p2.id, proposal_2_id);

    assert_eq!(p1.ledger, 100);
    assert_eq!(p2.ledger, 200);
    
    // outdate the first
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
    });
    
    let all_proposals = client.get_active_proposals();
    assert_eq!(all_proposals.len(), 1);
    let p = all_proposals.get_unchecked(0).unwrap();
    assert_eq!(p.id, proposal_2_id);
}


#[test]
#[should_panic(expected = "max number of proposals reached")]
fn max_number_of_proposals() {
    let client = create_client();
    let dao_id = "DIV".into_val(&client.env);

    client.create_proposal(&dao_id, &"P1".into_val(&client.env));
    client.create_proposal(&dao_id, &"P2".into_val(&client.env));
    client.create_proposal(&dao_id, &"P3".into_val(&client.env));
    client.create_proposal(&dao_id, &"P4".into_val(&client.env));
    client.create_proposal(&dao_id, &"P5".into_val(&client.env));
    client.create_proposal(&dao_id, &"P6".into_val(&client.env));
    client.create_proposal(&dao_id, &"P7".into_val(&client.env));
    client.create_proposal(&dao_id, &"P8".into_val(&client.env));
    client.create_proposal(&dao_id, &"P9".into_val(&client.env));
    client.create_proposal(&dao_id, &"P10".into_val(&client.env));
    client.create_proposal(&dao_id, &"P11".into_val(&client.env));
    client.create_proposal(&dao_id, &"P12".into_val(&client.env));
    client.create_proposal(&dao_id, &"P13".into_val(&client.env));
    client.create_proposal(&dao_id, &"P14".into_val(&client.env));
    client.create_proposal(&dao_id, &"P15".into_val(&client.env));
    client.create_proposal(&dao_id, &"P16".into_val(&client.env));
    client.create_proposal(&dao_id, &"P17".into_val(&client.env));
    client.create_proposal(&dao_id, &"P18".into_val(&client.env));
    client.create_proposal(&dao_id, &"P19".into_val(&client.env));
    client.create_proposal(&dao_id, &"P20".into_val(&client.env));
    client.create_proposal(&dao_id, &"P21".into_val(&client.env));
    client.create_proposal(&dao_id, &"P22".into_val(&client.env));
    client.create_proposal(&dao_id, &"P23".into_val(&client.env));
    client.create_proposal(&dao_id, &"P24".into_val(&client.env));
    client.create_proposal(&dao_id, &"P25".into_val(&client.env));
    client.create_proposal(&dao_id, &"P26".into_val(&client.env));
}
