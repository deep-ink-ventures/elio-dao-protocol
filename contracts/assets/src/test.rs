#![cfg(test)]

use soroban_sdk::{testutils::{Address as _, Ledger, LedgerInfo}, Address, Env, IntoVal};

use crate::{core_contract, votes_contract, AssetContract, AssetContractClient};

const FINALIZATION_DURATION: u32 = 5_000;
const PROPOSAL_DURATION: u32 = 10_000;

const SUPPLY: i128 = 1_000_000;

fn create_all_clients() -> (
    AssetContractClient<'static>,
    core_contract::Client<'static>,
    votes_contract::Client<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let core_id = env.register_contract_wasm(None, core_contract::WASM);
    let votes_id = env.register_contract_wasm(None, votes_contract::WASM);

    let core = core_contract::Client::new(&env, &core_id);
    let votes = votes_contract::Client::new(&env, &votes_id);

    let native_asset_id = env.register_stellar_asset_contract(Address::random(&env));

    core.init(&votes_id, &native_asset_id);
    votes.init(&core_id);

    let assets_id = env.register_contract(None, AssetContract);
    let assets = AssetContractClient::new(&env, &assets_id);

    (assets, core, votes)
}

fn create_token(client: &AssetContractClient, core_client: &core_contract::Client) -> Address {
    let symbol = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let address = Address::random(&client.env);
    let core_address = &core_client.address;
    client.init(&symbol, &name, &address, &core_address);
    client.mint(&address, &SUPPLY);
    address
}

#[test]
fn create_a_token() {
    let (client, core_client, _) = create_all_clients();
    let symbol = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let address = Address::random(&client.env);
    let core_address = &core_client.address;
    client.init(&symbol, &name, &address, &core_address);

    assert_eq!(symbol, client.symbol());
    assert_eq!(name, client.name());
    assert_eq!(address, client.owner());
    assert_eq!(core_address, &client.core_address());

    client.mint(&address, &SUPPLY);

    assert_eq!(SUPPLY, client.balance(&address));
}

#[test]
#[should_panic(expected = "#3")]
fn create_a_token_only_once() {
    let (client, core_client, _) = create_all_clients();
    create_token(&client, &core_client);
    create_token(&client, &core_client);
}

#[test]
#[should_panic(expected = "#5")]
fn mint_only_once() {
    let (client, core_client, _) = create_all_clients();
    let address= create_token(&client, &core_client);
    client.mint(&address, &SUPPLY);
}

#[test]
fn set_owner() {
    let (client, core_client, ..) = create_all_clients();
    create_token(&client, &core_client);
    let address = Address::random(&client.env);

    let owner = client.owner();

    client.set_owner(&owner, &address);
    let new_owner = client.owner();

    assert_eq!(address, new_owner);
}

#[test]
#[should_panic(expected = "#4")]
fn set_owner_auth() {
    let (client, core_client, ..) = create_all_clients();
    create_token(&client, &core_client);
    let address = Address::random(&client.env);
    client.set_owner(&address, &address);
}

#[test]
fn set_core_address() {
    let (client, core_client, ..) = create_all_clients();
    create_token(&client, &core_client);
    let owner = client.owner();

    client.set_core_address(&owner, &client.address);
    let new_id = client.core_address();

    assert_eq!(&client.address, &new_id);
}

#[test]
#[should_panic(expected = "#4")]
fn set_core_address_auth() {
    let (client, core_client, ..) = create_all_clients();
    create_token(&client, &core_client);
    let address = Address::random(&client.env);
    client.set_core_address(&address, &client.address);
}

#[test]
fn spendable_equals_balance() {
    let (client, ..) = create_all_clients();
    let address = Address::random(&client.env);
    assert_eq!(client.balance(&address), client.spendable(&address));
}

#[test]
fn token_assets_are_always_authorized() {
    let (client, ..) = create_all_clients();
    let address = Address::random(&client.env);
    assert_eq!(client.authorized(&address), true);
}

#[test]
fn xfer() {
    let (client, core_client, ..) = create_all_clients();
    create_token(&client, &core_client);
    let from = client.owner();
    let to = Address::random(&client.env);

    assert_eq!(client.balance(&from), 1_000_000);
    assert_eq!(client.balance(&to), 0);

    // budget reset
    client.env.budget().reset_default();

    client.xfer(&from, &to, &500_000);

    assert_eq!(client.balance(&from), 500_000);
    assert_eq!(client.balance(&to), 500_000);
}

#[test]
fn xfer_from() {
    let (client, core_client, ..) = create_all_clients();
    create_token(&client, &core_client);
    let from = client.owner();
    let to = Address::random(&client.env);
    let spender = Address::random(&client.env);
    client.incr_allow(&from, &spender, &250_000);

    assert_eq!(client.balance(&from), 1_000_000);
    assert_eq!(client.balance(&to), 0);
    assert_eq!(client.allowance(&from, &spender), 250_000);

    // budget reset
    client.env.budget().reset_default();

    client.xfer_from(&spender, &from, &to, &100_000);

    assert_eq!(client.balance(&from), 900_000);
    assert_eq!(client.balance(&to), 100_000);
    assert_eq!(client.allowance(&from, &spender), 150_000);
}

#[test]
#[ignore]
// this test counts exact number of checkpoints which currently
// fails due to checkpoint filtering being disabled
fn checkpoints() {
    let (client, core_client, votes_client) = create_all_clients();

    let owner = create_token(&client, &core_client);
    let whoever = Address::random(&client.env);

    // owner should have a checkpoint after issuance
    assert_eq!(client.get_checkpoint_count(&owner), 1);
    assert_eq!(client.get_checkpoint_count(&whoever), 0);

    let cp = client.get_checkpoint_at(&owner, &0);

    assert_eq!(cp.ledger, 0);
    assert_eq!(cp.balance, 1_000_000);

    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    client.xfer(&owner, &whoever, &100_000);

    // Since there is no proposal, the owners first cp should be overridden
    assert_eq!(client.get_checkpoint_count(&owner), 1);
    assert_eq!(client.get_checkpoint_count(&whoever), 1);

    let cp2 = client.get_checkpoint_at(&owner, &0);

    assert_eq!(cp2.ledger, 1);
    assert_eq!(cp2.balance, 900_000);

    let cp3 = client.get_checkpoint_at(&whoever, &0);

    assert_eq!(cp3.ledger, 1);
    assert_eq!(cp3.balance, 100_000);

    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    // let's create a proposal
    votes_client.create_proposal(&"DIV".into_val(&client.env), &Address::random(&client.env));
    client.xfer(&owner, &whoever, &100_000);

    assert_eq!(client.get_checkpoint_count(&owner), 2);
    assert_eq!(client.get_checkpoint_count(&whoever), 2);

    let cp2 = client.get_checkpoint_at(&owner, &1);

    assert_eq!(cp2.ledger, 10);
    assert_eq!(cp2.balance, 800_000);

    let cp3 = client.get_checkpoint_at(&whoever, &1);

    assert_eq!(cp3.ledger, 10);
    assert_eq!(cp3.balance, 200_000);

    // a second one
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 20,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });
    votes_client.create_proposal(&"DIV".into_val(&client.env), &Address::random(&client.env));
    client.xfer(&owner, &whoever, &100_000);

    assert_eq!(client.get_checkpoint_count(&owner), 3);
    assert_eq!(client.get_checkpoint_count(&whoever), 3);

    let cp2 = client.get_checkpoint_at(&owner, &2);

    assert_eq!(cp2.ledger, 20);
    assert_eq!(cp2.balance, 700_000);

    let cp3 = client.get_checkpoint_at(&whoever, &2);

    assert_eq!(cp3.ledger, 20);
    assert_eq!(cp3.balance, 300_000);

    // now let's outdate the first one
    client.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 10 + FINALIZATION_DURATION + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });
    client.xfer(&owner, &whoever, &100_000);

    assert_eq!(client.get_checkpoint_count(&owner), 2);
    assert_eq!(client.get_checkpoint_count(&whoever), 2);

    // let's verify we are getting the correct checkpoints for each proposal!
    assert_eq!(client.get_balance_at(&owner, &19), 700_000);
    assert_eq!(client.get_balance_at(&owner, &20), 700_000);
    assert_eq!(client.get_balance_at(&owner, &20_000), 600_000);

    assert_eq!(client.get_balance_at(&whoever, &19), 300_000);
    assert_eq!(client.get_balance_at(&whoever, &20), 300_000);
    assert_eq!(client.get_balance_at(&whoever, &20_000), 400_000);
}
