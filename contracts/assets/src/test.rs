#![cfg(test)]

use soroban_sdk::{Env, testutils::Address as _, Address, IntoVal};

use crate::{AssetContract, AssetContractClient};

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
    client.initialize(&symbol, &name, &supply, &address);
    address
}

#[test]
fn create_a_token() {
    let client = create_client();
    let address = Address::random(&client.env);
    let supply = 1_000_000;
    let symbol = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    client.initialize(&symbol, &name, &supply, &address);

    assert_eq!(symbol, client.symbol());
    assert_eq!(name, client.name());
    assert_eq!(supply, client.balance(&address));
}

#[test]
#[should_panic(expected = "DAO already issued a token")]
fn create_a_token_only_once() {
    let client = create_client();
    create_token(&client);
    create_token(&client);
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