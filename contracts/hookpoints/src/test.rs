#![cfg(test)]

use soroban_sdk::{testutils::{Address as _}, contractimpl, contract, token, Address, BytesN, Env, IntoVal, Bytes, contracterror, panic_with_error};

use crate::{
    core_contract::{WASM as CoreWASM, Client as CoreClient},
    votes_contract::{WASM as VotesWASM, Client as VotesClient},
    assets_contract::{WASM as AssetsWASM, Client as AssetsClient},
};
use crate::interface::HookpointsTrait;

/// *** This is a simple contract that is just altering things a bit for us to get going with tests
#[contract]
pub struct TestHookpointsContract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum HookTestError {
    OnBeforeDestroyDao = 0,
    OnBeforeChangeOwner = 1,
    OnBeforeProposalCreation = 2,
    OnBeforeSetMetadata = 3,
    OnBeforeFaultProposal = 4,
    OnBeforeFinalizeProposal = 5,
    OnBeforeMarkImplemented = 6,
}

#[contractimpl]
impl HookpointsTrait for TestHookpointsContract {
    fn on_before_destroy_dao(env: Env, _dao_id: Bytes) {
        panic_with_error!(env, HookTestError::OnBeforeDestroyDao)
    }

    fn on_before_change_owner(env: Env, _dao_id: Bytes, _new_owner: Address, _dao_owner: Address) {
        panic_with_error!(env, HookTestError::OnBeforeChangeOwner)
    }
    
    fn on_vote(_env: Env, _dao_id: Bytes, _proposal_id: u32, _account_id: Address, amount: i128) -> i128 {
        amount * 10
    }

    fn on_before_proposal_creation(env: Env, _dao_id: Bytes, _proposal_owner: Address) {
        panic_with_error!(env, HookTestError::OnBeforeProposalCreation)
    }

    fn on_before_set_metadata(env: Env, _dao_id: Bytes, _proposal_id: u32, _meta: Bytes, _hash: Bytes, _proposal_owner: Address) {
        panic_with_error!(env, HookTestError::OnBeforeSetMetadata)
    }

    fn on_set_configuration(_env: Env, _dao_id: Bytes, proposal_duration: u32) -> u32 {
        proposal_duration + 10
    }

    fn on_before_fault_proposal(env: Env, _dao_id: Bytes, _proposal_id: u32, _reason: Bytes) {
        panic_with_error!(env, HookTestError::OnBeforeFaultProposal)
    }

    fn on_before_finalize_proposal(env: Env, _dao_id: Bytes, _proposal_id: u32) {
        panic_with_error!(env, HookTestError::OnBeforeFinalizeProposal)
    }

    fn on_before_mark_implemented(env: Env, _dao_id: Bytes, _proposal_id: u32) {
        panic_with_error!(env, HookTestError::OnBeforeMarkImplemented)
    }

    fn on_incr_allowance(_env: Env, _from: Address, _spender: Address, amount: i128) -> i128 {
        amount + 20
    }

    fn on_decr_allowance(_env: Env, _from: Address, _spender: Address, amount: i128) -> i128 {
        amount + 30
    }

    fn on_xfer(_env: Env, _from: Address, _to: Address, amount: i128) -> i128 {
        amount + 40
    }

    fn on_xfer_from(_env: Env, _spender: Address, _from: Address, _to: Address, amount: i128) -> i128 {
        amount + 50
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
fn should_remove_hookpoint() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.core.remove_hookpoint(&protocol.dao_id, &protocol.dao_owner);
    let voting_power = protocol.votes.vote(&protocol.dao_id, &protocol.proposal_id, &true, &protocol.dao_owner);
    assert_eq!(voting_power, MINT);
}

#[test]
#[should_panic(expected="#0")]
fn should_respect_contract_on_before_destroy_dao_dao() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.core.destroy_dao(&protocol.dao_id, &protocol.dao_owner);
}

#[test]
#[should_panic(expected="#1")]
fn should_respect_contract_on_before_change_owner() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    let whoever = Address::random(&protocol.env);
    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.core.change_owner(&protocol.dao_id, &whoever, &protocol.dao_owner);
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
#[should_panic(expected="#2")]
fn should_respect_contract_on_before_proposal_creation() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.votes.create_proposal(&protocol.dao_id, &protocol.dao_owner);
}

#[test]
#[should_panic(expected="#3")]
fn should_respect_contract_on_before_set_metadata() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    let meta = ("meta").into_val(&protocol.env);
    let hash = ("hash").into_val(&protocol.env);
    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.votes.set_metadata(&protocol.dao_id, &protocol.proposal_id, &meta, &hash, &protocol.dao_owner);
}

#[test]
fn should_respect_contract_on_set_configuration() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    let proposal_duration: u32 = 10_000;
    let min_threshold_configuration: i128 = 1_000;
    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    let configuration = protocol.votes.set_configuration(
        &protocol.dao_id,
        &proposal_duration,
        &min_threshold_configuration,
        &protocol.dao_owner
    );
    assert_eq!(configuration.proposal_duration + 10, proposal_duration + 10)
}

#[test]
#[should_panic(expected="#4")]
fn should_respect_contract_on_before_fault_proposal() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    let reason = ("reason").into_val(&protocol.env);
    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.votes.fault_proposal(&protocol.dao_id, &protocol.proposal_id, &reason, &protocol.dao_owner);
}

#[test]
#[should_panic(expected="#5")]
fn should_respect_contract_on_before_finalize_proposal() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.votes.finalize_proposal(&protocol.dao_id, &protocol.proposal_id);
}

#[test]
#[ignore] // WasmVM, InternalError
#[should_panic(expected="#5")]
fn should_respect_contract_on_before_mark_implemented() {
    let protocol = Protocol::new();
    let hookpoints_address = protocol.env.register_contract(None, TestHookpointsContract);

    protocol.core.set_hookpoint(&protocol.dao_id, &hookpoints_address, &protocol.dao_owner);
    protocol.votes.mark_implemented(&protocol.proposal_id, &protocol.dao_owner);
}