#![cfg(test)]

mod votes_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_votes.wasm");
}

mod assets_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_assets.wasm");
}

use soroban_sdk::{log, testutils::Address as _, Address, BytesN, Env, IntoVal};

use crate::{types::Dao, CoreContract, CoreContractClient};

fn create_client() -> CoreContractClient<'static> {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, CoreContract);
    let votes_id = env.register_contract_wasm(None, votes_contract::WASM);

    let client = CoreContractClient::new(&env, &contract_id);

    client.init(&votes_id);

    client
}

fn create_dao(client: &CoreContractClient) -> Dao {
    let env = &client.env;
    let id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let owner = Address::random(env);
    log!(env, "creating DAO");
    client.create_dao(&id, &name, &owner)
}

#[test]
#[should_panic(expected = "Already initialized")]
fn cannot_initialize_twice() {
    let client = create_client();
    let fake_id = Address::random(&client.env);
    client.init(&fake_id);
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
#[should_panic(expected = "DAO does not exist")]
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
fn set_metadata() {
    let client = create_client();
    let dao = create_dao(&client);

    let url = "https://deep-ink.ventures".into_val(&client.env);
    let hash =
        "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&client.env);

    client.set_metadata(&dao.id, &url, &hash, &dao.owner);

    let meta = client.get_metadata(&dao.id);
    assert_eq!(meta.url, url);
    assert_eq!(meta.hash, hash);
}

#[test]
#[should_panic(expected = "Address not DAO Owner")]
fn set_metadata_only_owner() {
    let client = create_client();
    let dao = create_dao(&client);
    let whoever = Address::random(&client.env);

    let url = "https://deep-ink.ventures".into_val(&client.env);
    let hash =
        "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&client.env);

    client.set_metadata(&dao.id, &url, &hash, &whoever);
}

#[test]
#[should_panic(expected = "metadata does not exist")]
fn non_existing_meta_panics() {
    let client = create_client();
    let dao = create_dao(&client);

    client.get_metadata(&dao.id);
}

#[test]
fn issue_token_once() {
    let client = create_client();
    let env = &client.env;

    log!(env, "installing assets contract WASM");
    let assets_wasm_hash = env.install_contract_wasm(assets_contract::WASM);

    let salt = BytesN::from_array(env, &[0; 32]);
    let dao = create_dao(&client);

    log!(env, "issuing token");
    client.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt);

    log!(env, "getting DAO asset id");
    let asset_id = client.get_dao_asset_id(&dao.id);
    let asset_client = assets_contract::Client::new(env, &asset_id);
    assert_eq!(dao.id, asset_client.symbol());
    assert_eq!(dao.name, asset_client.name());
    assert_eq!(dao.owner, asset_client.owner());
    assert_eq!(client.address, asset_client.governance_id());

    log!(env, "minting token");
    let supply = 1_000_000;
    asset_client.mint(&dao.owner, &supply);
    assert_eq!(supply, asset_client.balance(&dao.owner));
}

#[test]
#[should_panic(expected = "asset already issued")]

fn cannot_issue_token_twice() {
    let client = create_client();

    let assets_wasm_hash = &client.env.install_contract_wasm(assets_contract::WASM);
    let dao = create_dao(&client);

    let salt = BytesN::from_array(&client.env, &[0; 32]);
    let salt2 = BytesN::from_array(&client.env, &[1; 32]);
    client.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt);
    client.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt2);
}

#[test]
#[should_panic(expected = "asset not issued")]
fn cannot_get_asset_id_if_non_existing() {
    let client = create_client();
    let dao = create_dao(&client);
    client.get_dao_asset_id(&dao.id);
}
