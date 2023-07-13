#![cfg(test)]

mod votes_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_votes.wasm");
}

mod assets_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_assets.wasm");
}

use soroban_sdk::{log, testutils::Address as _, token, Address, BytesN, Env, IntoVal};

use crate::{types::Dao, CoreContract, CoreContractClient};
use votes_contract::Client as VotesContractClient;

struct Clients {
    core: CoreContractClient<'static>,
    votes: VotesContractClient<'static>,
    native_asset: token::Client<'static>,
}

fn create_clients() -> Clients {
    let env = Env::default();
    env.mock_all_auths();

    let core_id = env.register_contract(None, CoreContract);
    let votes_id = env.register_contract_wasm(None, votes_contract::WASM);

    let core = CoreContractClient::new(&env, &core_id);
    let votes = VotesContractClient::new(&env,&votes_id);

    let native_asset_id = env.register_stellar_asset_contract(Address::random(&env));
    let native_asset = token::Client::new(&env, &native_asset_id);

    core.init(&votes_id, &native_asset_id);
    votes.init(&core_id);
    Clients { core, votes,  native_asset }
}

fn create_dao(core: &CoreContractClient<'static>, dao_owner: &Address) -> Dao {
    let env = &core.env;

    let id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);

    core.create_dao(&id, &name, &dao_owner)
}

fn mint_and_create_dao(clients: &Clients, dao_owner: &Address) -> Dao {
    clients.native_asset.mint(&dao_owner, &i128::MAX);
    create_dao(&clients.core, &dao_owner)
}

#[test]
#[should_panic(expected = "Status(ContractError(3))")]
fn cannot_initialize_twice() {
    let core = create_clients().core;
    let fake_id = Address::random(&core.env);
    core.init(&fake_id, &fake_id);
}

#[test]
fn create_a_dao() {
    let clients = create_clients();

    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    clients.native_asset.mint(&user, &i128::MAX);

    let id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);
    let balance_before = clients.native_asset.balance(&user);
    core.create_dao(&id, &name, &user);
    let balance_after = clients.native_asset.balance(&user);
    assert!(balance_after < balance_before);

    let dao = core.get_dao(&"DIV".into_val(env));
    assert_eq!(dao.id, id);
    assert_eq!(dao.name, name);
    assert_eq!(dao.owner, user);
}

#[test]
#[ignore]
#[should_panic(expected = "balance is not sufficient to spend")]
fn cannot_create_a_dao_without_funds() {
    let core = create_clients().core;
    create_dao(&core, &Address::random(&core.env));
}

#[test]
#[should_panic(expected = "Status(ContractError(1))")]
fn cannot_create_a_dao_twice() {
    let clients = create_clients();
    let core = &clients.core;
    let user = Address::random(&core.env);

    mint_and_create_dao(&clients, &user);
    create_dao(&core, &user);
}

#[test]
#[should_panic(expected = "Status(ContractError(2))")]
fn destroy_a_dao() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);

    let dao = mint_and_create_dao(&clients, &user);
    let balance_before = clients.native_asset.balance(&user);

    core.destroy_dao(&dao.id, &user);
    let balance_after = clients.native_asset.balance(&user);
    assert!(balance_after > balance_before);

    core.get_dao(&dao.id);
}

#[test]
#[should_panic(expected = "Status(ContractError(1009))")]
fn destroy_a_dao_destroys_configuration() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);

    let dao = mint_and_create_dao(&clients, &user);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = votes_contract::Voting::MAJORITY;
    clients.votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    core.destroy_dao(&dao.id, &user);

    clients.votes.get_configuration(&dao.id);
}


#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn destroy_a_dao_only_as_owner() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);

    let dao = mint_and_create_dao(&clients, &user);
    core.destroy_dao(&dao.id, &Address::random(env));
}

#[test]
fn change_dao_owner() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    let new_owner = Address::random(&core.env);
    core.change_owner(&dao.id, &new_owner, &dao.owner);
    assert_eq!(core.get_dao(&dao.id).owner, new_owner);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn change_dao_owner_only_as_owner() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    let new_owner = Address::random(&core.env);
    core.change_owner(&dao.id, &new_owner, &new_owner);
}

#[test]
fn set_metadata() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    let url = "https://deep-ink.ventures".into_val(&core.env);
    let hash =
        "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&core.env);

    core.set_metadata(&dao.id, &url, &hash, &dao.owner);

    let meta = core.get_metadata(&dao.id);
    assert_eq!(meta.url, url);
    assert_eq!(meta.hash, hash);
}

#[test]
#[should_panic(expected = "Status(ContractError(4))")]
fn set_metadata_only_owner() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    let url = "https://deep-ink.ventures".into_val(&core.env);
    let hash =
        "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(&core.env);

    let whoever = Address::random(&core.env);
    core.set_metadata(&dao.id, &url, &hash, &whoever);
}

#[test]
#[should_panic(expected = "Status(ContractError(7))")]
fn non_existing_meta_panics() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    core.get_metadata(&dao.id);
}

#[test]
fn issue_token_once() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    log!(env, "installing assets contract WASM");
    let assets_wasm_hash = env.install_contract_wasm(assets_contract::WASM);

    log!(env, "issuing token");
    let salt = BytesN::from_array(env, &[0; 32]);
    core.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt);

    log!(env, "getting DAO asset id");
    let asset_id = core.get_dao_asset_id(&dao.id);
    let asset_core = assets_contract::Client::new(env, &asset_id);
    assert_eq!(dao.id, asset_core.symbol());
    assert_eq!(dao.name, asset_core.name());
    assert_eq!(dao.owner, asset_core.owner());
    assert_eq!(core.address, asset_core.core_address());

    log!(env, "minting token");
    let supply = 1_000_000;
    asset_core.mint(&dao.owner, &supply);
    assert_eq!(supply, asset_core.balance(&dao.owner));
}

#[test]
#[should_panic(expected = "Status(ContractError(5))")]
fn cannot_issue_token_twice() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);

    log!(env, "installing assets contract WASM");
    let assets_wasm_hash = env.install_contract_wasm(assets_contract::WASM);

    log!(env, "issuing token twice");
    let salt = BytesN::from_array(&core.env, &[0; 32]);
    let salt2 = BytesN::from_array(&core.env, &[1; 32]);
    core.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt);
    core.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt2);
}

#[test]
#[should_panic(expected = "Status(ContractError(6))")]
fn cannot_get_asset_id_if_non_existing() {
    let clients = create_clients();
    let core = &clients.core;
    let env = &core.env;
    let user = Address::random(env);
    let dao = mint_and_create_dao(&clients, &user);
    core.get_dao_asset_id(&dao.id);
}
