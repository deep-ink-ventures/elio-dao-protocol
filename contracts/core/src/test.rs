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
#[should_panic(expected = "DAO does not exists")]
fn destroy_a_dao() {
    let client = create_client();
    
    let dao = create_dao(&client);
    
    client.destroy_dao(&dao.id, &dao.owner);
    client.get_dao(&dao.id);
}

#[test]
fn change_dao_owner() {
    let client = create_client();
    let new_owner = Address::random(&client.env);
    let dao = create_dao(&client);

    let dao = client.change_owner(&dao.id, &new_owner, &dao.owner);
    assert_eq!(client.get_dao(&dao.id).owner, new_owner);
}
