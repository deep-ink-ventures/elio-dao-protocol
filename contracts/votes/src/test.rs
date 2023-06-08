#![cfg(test)]

use soroban_sdk::{
    log,
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, BytesN, Env, IntoVal,
};

use crate::{
    assets_contract, core_contract,
    types::{PropStatus, FINALIZATION_DURATION, PROPOSAL_DURATION, PROPOSAL_MAX_NR},
    VotesContract, VotesContractClient,
};

fn create_clients() -> (core_contract::Client<'static>, VotesContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let core_id = env.register_contract_wasm(None, core_contract::WASM);
    let id = env.register_contract(None, VotesContract);

    let core_client = core_contract::Client::new(&env, &core_id);
    let client = VotesContractClient::new(&env, &id);

    core_client.init(&id);
    client.init(&core_id);

    (core_client, client)
}

fn create_client() -> VotesContractClient<'static> {
    create_clients().1
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
        sequence_number: 100 + PROPOSAL_DURATION + FINALIZATION_DURATION + 1,
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
    let (core, client) = create_clients();
    let env = &client.env;
    let dao_id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let dao_owner = Address::random(env);
    log!(env, "creating DAO");
    core.create_dao(&dao_id, &name, &dao_owner);

    let assets_wasm_hash = env.install_contract_wasm(assets_contract::WASM);
    let salt = BytesN::from_array(env, &[1; 32]);
    log!(env, "issuing DAO token");
    core.issue_token(&dao_id, &dao_owner, &assets_wasm_hash, &salt);

    let asset_id = core.get_dao_asset_id(&dao_id);
    let asset = assets_contract::Client::new(env, &asset_id);
    let supply = 1_000_000;
    log!(env, "minting DAO token");
    asset.mint(&dao_owner, &supply);

    let owner = Address::random(env);
    log!(env, "creating proposal");
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let voter = dao_owner;
    client.vote(&dao_id, &proposal_id, &true, &voter);
    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.in_favor, supply);
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
    let (core_client, client) = create_clients();
    let env = &client.env;
    let dao_id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let dao_owner = Address::random(env);
    core_client.create_dao(&dao_id, &name, &dao_owner);

    let owner = Address::random(env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let reason = "bad".into_val(env);
    client.fault_proposal(&dao_id, &proposal_id, &reason, &dao_owner);

    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.inner.status, PropStatus::Faulty(reason));
}

#[test]
#[should_panic(expected = "not the DAO owner")]
fn mark_faulty_only_owner() {
    let (core_client, client) = create_clients();
    let env = &client.env;
    let dao_id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let dao_owner = Address::random(env);
    core_client.create_dao(&dao_id, &name, &dao_owner);

    let owner = Address::random(env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let reason = "bad".into_val(env);
    client.fault_proposal(&dao_id, &proposal_id, &reason, &Address::random(env));
}

#[test]
fn finalize() {
    let (core, client) = create_clients();
    let env = &client.env;
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
    });
    let dao_id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let dao_owner = Address::random(env);
    core.create_dao(&dao_id, &name, &dao_owner);

    let owner = Address::random(env);
    let proposal_id = client.create_proposal(&dao_id, &owner);

    // make finalization possible
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
    });

    client.finalize_proposal(&dao_id, &proposal_id);

    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.inner.status, PropStatus::Rejected);
    assert_eq!(proposal.inner, client.get_archived_proposal(&proposal_id));
}

#[test]
fn mark_implemented() {
    let (core, client) = create_clients();
    let env = &client.env;
    let dao_id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let dao_owner = Address::random(env);
    log!(env, "creating DAO");
    core.create_dao(&dao_id, &name, &dao_owner);

    let assets_wasm_hash = env.install_contract_wasm(assets_contract::WASM);
    let salt = BytesN::from_array(env, &[1; 32]);
    log!(env, "issuing DAO token");
    core.issue_token(&dao_id, &dao_owner, &assets_wasm_hash, &salt);

    let asset_id = core.get_dao_asset_id(&dao_id);
    let asset = assets_contract::Client::new(env, &asset_id);
    let supply = 1_000_000;
    log!(env, "minting DAO token");
    asset.mint(&dao_owner, &supply);

    let owner = Address::random(env);
    log!(env, "creating proposal");
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let voter = dao_owner.clone();
    client.vote(&dao_id, &proposal_id, &true, &voter);

    // make finalization possible
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
    });

    client.finalize_proposal(&dao_id, &proposal_id);

    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.inner.status, PropStatus::Accepted);
    assert_eq!(proposal.inner, client.get_archived_proposal(&proposal_id));

    client.mark_implemented(&proposal_id, &dao_owner);

    let proposal = client.get_archived_proposal(&proposal_id);
    assert_eq!(proposal.status, PropStatus::Implemented);
}

#[test]
#[should_panic(expected = "not the DAO owner")]
fn mark_implemented_only_owner() {
    let (core, client) = create_clients();
    let env = &client.env;
    let dao_id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let dao_owner = Address::random(env);
    log!(env, "creating DAO");
    core.create_dao(&dao_id, &name, &dao_owner);

    let assets_wasm_hash = env.install_contract_wasm(assets_contract::WASM);
    let salt = BytesN::from_array(env, &[1; 32]);
    log!(env, "issuing DAO token");
    core.issue_token(&dao_id, &dao_owner, &assets_wasm_hash, &salt);

    let asset_id = core.get_dao_asset_id(&dao_id);
    let asset = assets_contract::Client::new(env, &asset_id);
    let supply = 1_000_000;
    log!(env, "minting DAO token");
    asset.mint(&dao_owner, &supply);

    let owner = Address::random(env);
    log!(env, "creating proposal");
    let proposal_id = client.create_proposal(&dao_id, &owner);

    let voter = dao_owner.clone();
    client.vote(&dao_id, &proposal_id, &true, &voter);

    // make finalization possible
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
    });

    client.finalize_proposal(&dao_id, &proposal_id);

    let proposal = client
        .get_active_proposals(&dao_id)
        .get_unchecked(0)
        .unwrap();
    assert_eq!(proposal.inner.status, PropStatus::Accepted);
    assert_eq!(proposal.inner, client.get_archived_proposal(&proposal_id));

    client.mark_implemented(&proposal_id, &Address::random(env));
}
