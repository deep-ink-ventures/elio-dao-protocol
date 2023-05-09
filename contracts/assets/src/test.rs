#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Bytes, Env, IntoVal};

use crate::{AssetContract, AssetContractClient, Token};

fn create_client() -> AssetContractClient {
    let env = Env::default();
    let contract_id = env.register_contract(None, AssetContract);
    AssetContractClient::new(&env, &contract_id)
}

fn create_token(client: &AssetContractClient) -> Address {
    let symbol = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let address = Address::random(&client.env);
    let supply = 1_000_000;
    let governance_id = Bytes::from_array(&client.env, &[0; 32]).try_into().unwrap();
    client.init(&symbol, &name, &supply, &address, &governance_id);
    address
}

#[test]
fn create_a_token() {
    let client = create_client();
    let address = Address::random(&client.env);
    let supply = 1_000_000;
    let symbol = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let governance_id = Bytes::from_array(&client.env, &[0; 32]).try_into().unwrap();
    client.init(&symbol, &name, &supply, &address, &governance_id);

    assert_eq!(symbol, client.symbol());
    assert_eq!(name, client.name());
    assert_eq!(supply, client.balance(&address));
    assert_eq!(address, client.owner());
    assert_eq!(governance_id, client.governance_id());
}

#[test]
#[should_panic(expected = "DAO already issued a token")]
fn create_a_token_only_once() {
    let client = create_client();
    create_token(&client);
    create_token(&client);
}

#[test]
fn set_owner() {
    let client = create_client();
    create_token(&client);
    let address = Address::random(&client.env);

    let owner = client.owner();

    client.set_owner(&owner, &address);
    let new_owner = client.owner();

    assert_eq!(address, new_owner);
}

#[test]
#[should_panic(expected = "not Token owner")]
fn set_owner_auth() {
    let client = create_client();
    create_token(&client);
    let address = Address::random(&client.env);
    client.set_owner(&address, &address);
}

#[test]
fn set_governance_id() {
    let client = create_client();
    create_token(&client);
    let owner = client.owner();

    client.set_governance_id(&owner, &client.contract_id);
    let new_id = client.governance_id();

    assert_eq!(&client.contract_id, &new_id);
}

#[test]
#[should_panic(expected = "not Token owner")]
fn set_governance_id_auth() {
    let client = create_client();
    create_token(&client);
    let address = Address::random(&client.env);
    client.set_governance_id(&address, &client.contract_id);
}

#[test]
fn spendable_equals_balance() {
    let client = create_client();
    let address = Address::random(&client.env);
    assert_eq!(client.balance(&address), client.spendable(&address));
}

#[test]
fn token_assets_are_always_authoritzed() {
    let client = create_client();
    let address = Address::random(&client.env);
    assert_eq!(client.authorized(&address), true);
}

#[test]
fn allowances() {
    let client = create_client();
    let address = create_a_token();
}

#[test]
fn xfer() {
    let client = create_client();
    let from = create_token(&client);
    let to = Address::random(&client.env);

    assert_eq!(client.balance(&from), 1_000_000);
    assert_eq!(client.balance(&to), 0);

    client.xfer(&from, &to, &500_000);

    assert_eq!(client.balance(&from), 500_000);
    assert_eq!(client.balance(&to), 500_000);
}

#[test]
fn xfer_from() {
    let client = create_client();
    let from = create_token(&client);
    let to = Address::random(&client.env);
    let spender = Address::random(&client.env);
    client.incr_allow(&from, &spender, &250_000);

    assert_eq!(client.balance(&from), 1_000_000);
    assert_eq!(client.balance(&to), 0);
    assert_eq!(client.allowance(&from, &spender), 250_000);

    client.xfer_from(&spender, &from, &to, &100_000);

    assert_eq!(client.balance(&from), 900_000);
    assert_eq!(client.balance(&to), 100_000);
    assert_eq!(client.allowance(&from, &spender), 150_000);
}
