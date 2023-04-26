#![cfg(test)]

use soroban_sdk::{Env, testutils::Address as _, Address, IntoVal};

use crate::{CoreContract, CoreContractClient, types::Dao};

fn create_client() -> CoreContractClient {
    let env = Env::default();
    let contract_id = env.register_contract(None, CoreContract);
    CoreContractClient::new(&env, &contract_id)
}

fn create_dao(client: &CoreContractClient) -> Dao {
    let id = "DIV".into_val(&client.env);
    let name = "Deep Ink Ventures".into_val(&client.env);
    let owner = Address::random(&client.env);
    client.create_dao(&id, &name, &owner)
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