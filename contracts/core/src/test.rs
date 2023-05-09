#![cfg(test)]

mod votes_contract {
    soroban_sdk::contractimport!(
        file = "../../wasm/elio_votes.wasm"
    );
}

mod assets_contract {
    soroban_sdk::contractimport!(
        file = "../../wasm/elio_assets.wasm"
    );
}

use soroban_sdk::{Env, testutils::Address as _, Address, IntoVal, log};

use crate::{CoreContract, CoreContractClient, types::Dao};

fn create_client() -> CoreContractClient {
    let env = Env::default();
    let contract_id = env.register_contract(None, CoreContract);
    let client = CoreContractClient::new(&env, &contract_id);
    init_client(&client);
    client
}

fn init_client(client: &CoreContractClient) {
    // install votes
    let votes_wasm_hash = &client.env.install_contract_wasm(votes_contract::WASM);
    client.init(&votes_wasm_hash);
}

fn create_dao(client: &CoreContractClient) -> Dao {
    let id = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let owner = Address::random(&client.env);
    client.create_dao(&id, &name, &owner)
}

#[test]
#[should_panic(expected = "Already initialized")]
fn cannot_initialize_twice() {
    let client = create_client();
    init_client(&client);
}

#[test]
fn create_a_dao() {
    let client = create_client();

    let id = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let owner = Address::random(&client.env);
    client.create_dao(&id, &name, &owner);

    let dao = client.get_dao(&"DIV".into_val(&client.env));
    assert_eq!(dao.id, id);
    assert_eq!(dao.name, name);
    assert_eq!(dao.owner, owner);
}

#[test]
#[should_panic(expected = "DAO already exists")]
fn cannot_create_a_dao_twice() {
    let client = create_client();
    create_dao(&client);
    create_dao(&client);
}

#[test]
#[should_panic(expected = "DAO does not exists")]
fn destroy_a_dao() {
    let client = create_client();

    let dao = create_dao(&client);

    client.destroy_dao(&dao.id, &dao.owner);
    client.get_dao(&dao.id);
}

#[test]
#[should_panic(expected = "Address not DAO Owner")]
fn destroy_a_dao_only_as_owner() {
    let client = create_client();

    let dao = create_dao(&client);
    client.destroy_dao(&dao.id, &Address::random(&client.env));
}

#[test]
fn change_dao_owner() {
    let client = create_client();
    let new_owner = Address::random(&client.env);
    let dao = create_dao(&client);

    let dao = client.change_owner(&dao.id, &new_owner, &dao.owner);
    assert_eq!(client.get_dao(&dao.id).owner, new_owner);
}

#[test]
#[should_panic(expected = "Address not DAO Owner")]
fn change_dao_owner_only_as_owner() {
    let client = create_client();
    let new_owner = Address::random(&client.env);
    let dao = create_dao(&client);

    client.change_owner(&dao.id, &new_owner, &new_owner);
}

#[test]
fn set_meta_data() {
    let client = create_client();
    let dao = create_dao(&client);

    let url = "https://deep-ink.ventures".into_val(&client.env);
    let hash = "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&client.env);

    client.set_meta_data(&dao.id, &url, &hash, &dao.owner);

    let meta = client.get_meta_data(&dao.id);
    assert_eq!(meta.url, url);
    assert_eq!(meta.hash, hash);
}

#[test]
#[should_panic(expected = "Address not DAO Owner")]
fn set_meta_data_only_owner() {
    let client = create_client();
    let dao = create_dao(&client);
    let whoever = Address::random(&client.env);

    let url = "https://deep-ink.ventures".into_val(&client.env);
    let hash = "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&client.env);

    client.set_meta_data(&dao.id, &url, &hash, &whoever);
}

#[test]
#[should_panic(expected = "MetaData does not exists")]
fn non_existing_meta_panics() {
    let client = create_client();
    let dao = create_dao(&client);

    client.get_meta_data(&dao.id);
}

#[test]
fn issue_token() {
    let client = create_client();

    let assets_wasm_hash = &client.env.install_contract_wasm(assets_contract::WASM);

    let salt = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".into_val(&client.env);
    let dao = create_dao(&client);
    client.issue_token(&dao.id, &1_000_000, &dao.owner, &assets_wasm_hash, &salt);

    let asset_id = client.get_dao_asset_id(&dao.id);
    let asset_client = assets_contract::Client::new(&client.env, &asset_id);
    assert_eq!(dao.id, asset_client.symbol());
    assert_eq!(dao.name, asset_client.name());
    assert_eq!(1_000_000, asset_client.balance(&dao.owner));
    assert_eq!(dao.owner, asset_client.owner());
    assert_eq!(client.contract_id, asset_client.governance_id());
}

#[test]
#[should_panic(expected = "asset already issued")]
fn cant_issue_token_twice() {
    let client = create_client();

    let assets_wasm_hash = &client.env.install_contract_wasm(assets_contract::WASM);

    let salt = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".into_val(&client.env);
    let dao = create_dao(&client);
    let salt2 = "YYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY".into_val(&client.env);
    client.issue_token(&dao.id, &1_000_000, &dao.owner, &assets_wasm_hash, &salt);
    client.issue_token(&dao.id, &1_000_000, &dao.owner, &assets_wasm_hash, &salt2);
}

#[test]
#[should_panic(expected = "asset not issued")]
fn cant_get_asset_id_if_non_existing() {
    let client = create_client();
    let dao = create_dao(&client);
    client.get_dao_asset_id(&dao.id);
}