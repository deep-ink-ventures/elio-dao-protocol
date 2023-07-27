#![cfg(test)]

use soroban_sdk::{testutils::{Address as _}, contractimpl, contract, token, Address, BytesN, Env, IntoVal, Bytes};

use crate::{
    core_contract::{WASM as CoreWASM, Client as CoreClient},
    votes_contract::{WASM as VotesWASM, Client as VotesClient},
    assets_contract::{WASM as AssetsWASM, Client as AssetsClient},
};
use crate::interface::HookpointsTrait;

/// *** This is a simple contract that is just altering things a bit for us to get going with tests
#[contract]
pub struct TestHookpointsContract;

#[contractimpl]
impl HookpointsTrait for TestHookpointsContract {

    fn on_vote(_env: Env, _dao_id: Bytes, _proposal_id: u32, _account_id: Address, amount: i128) -> i128 {
        amount * 10
    }

    fn on_before_proposal_creation(_env: Env, _dao_id: Bytes, _proposal_owner: Address) {
        todo!()
    }
}


const MINT: i128 = 1_000 * 10_000_000;
pub const MAX_I128: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

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

        env.budget().reset_unlimited();

        let dao_owner = Address::random(&env);

        let core_id = env.register_contract_wasm(None, CoreWASM);
        let votes_id = env.register_contract_wasm(None, VotesWASM);

        let core = CoreClient::new(&env, &core_id);
        let votes = VotesClient::new(&env, &votes_id);

        let native_asset_id = env.register_stellar_asset_contract(Address::random(&env));
        let native_asset_admin = token::AdminClient::new(&env, &native_asset_id);

        core.init(&votes_id, &native_asset_id);
        votes.init(&core_id);

        native_asset_admin.mint(&dao_owner, &MAX_I128);
        let dao_id = "DIV".into_val(&env);
        let dao_name = "Deep Ink Ventures".into_val(&env);
        core.create_dao(&dao_id, &dao_name, &dao_owner);

        let assets_wasm_hash = env.deployer().upload_contract_wasm(AssetsWASM);
        let salt = BytesN::from_array(&env, &[1; 32]);
        core.issue_token(&dao_id, &dao_owner, &assets_wasm_hash, &salt);

        let asset_id = core.get_dao_asset_id(&dao_id);
        let asset = AssetsClient::new(&env, &asset_id);
        asset.mint(&dao_owner, &MINT);

        let proposal_duration: u32 = 10_000;
        let min_threshold_configuration: i128 = 1_000;
        votes.set_configuration(
            &dao_id,
            &proposal_duration,
            &min_threshold_configuration,
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
