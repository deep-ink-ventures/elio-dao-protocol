#![cfg(test)]

use soroban_sdk::{testutils::{Address as _}, contractimpl, token, Address, BytesN, Env, IntoVal, Bytes};

use crate::{
    core_contract::{WASM as CoreWASM, Client as CoreClient},
    votes_contract::{WASM as VotesWASM, Client as VotesClient, Voting},
    assets_contract::{WASM as AssetsWASM, Client as AssetsClient},
};
use crate::interface::HookpointsTrait;


/// *** This is a simple contract that is just altering things a bit for us to get going with tests
pub struct TestHookpointsContract;

#[contractimpl]
impl HookpointsTrait for TestHookpointsContract {

    fn on_vote(_env: Env, _dao_id: Bytes, _proposal_id: u32, _account_id: Address, amount: i128) -> i128 {
        amount * 10
    }
}


const MINT: i128 = 1_000 * 10_000_000;

/// *** A fully working testing env for  tests
struct Protocol {
    env: Env,
    core: CoreClient<'static>,
    votes: VotesClient<'static>,
    proposal_id: u32,
    dao_id: Bytes,
    dao_owner: Address
}

impl Protocol {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let dao_owner = Address::random(&env);

        let core_id = env.register_contract_wasm(None, CoreWASM);
        let votes_id = env.register_contract_wasm(None, VotesWASM);

        let core = CoreClient::new(&env, &core_id);
        let votes = VotesClient::new(&env, &votes_id);

        let native_asset_id = env.register_stellar_asset_contract(Address::random(&env));
        let native_asset = token::Client::new(&env, &native_asset_id);

        core.init(&votes_id, &native_asset_id);
        votes.init(&core_id);

        native_asset.mint(&dao_owner, &i128::MAX);
        let dao_id = "DIV".into_val(&env);
        let dao_name = "Deep Ink Ventures".into_val(&env);
        core.create_dao(&dao_id, &dao_name, &dao_owner);

        let assets_wasm_hash = env.install_contract_wasm(AssetsWASM);
        let salt = BytesN::from_array(&env, &[1; 32]);
        core.issue_token(&dao_id, &dao_owner, &assets_wasm_hash, &salt);

        let asset_id = core.get_dao_asset_id(&dao_id);
        let asset = AssetsClient::new(&env, &asset_id);
        asset.mint(&dao_owner, &MINT);

        let proposal_duration: u32 = 10_000;
        let proposal_token_deposit: u128 = 100_000_000;
        let min_threshold_configuration: i128 = 1_000;
        let voting = Voting::MAJORITY;
        votes.set_configuration(
            &dao_id,
            &proposal_duration,
            &proposal_token_deposit,
            &min_threshold_configuration,
            &voting,
            &dao_owner
        );

        let proposal_id = votes.create_proposal(&dao_id, &dao_owner);

        Self {
            env,
            core,
            votes,
            dao_id,
            proposal_id,
            dao_owner
        }
    }
}

#[test]
fn should_respect_contract_on_vote() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    let voting_power = protocol.votes.vote(&protocol.dao_id, &protocol.proposal_id, &true, &protocol.dao_owner);
    assert_eq!(voting_power, MINT * 10);
}

#[test]
fn should_remove_hookpoint() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.core.remove_hookpoint(&protocol.dao_id, &protocol.dao_owner);
    let voting_power = protocol.votes.vote(&protocol.dao_id, &protocol.proposal_id, &true, &protocol.dao_owner);
    assert_eq!(voting_power, MINT);
}
