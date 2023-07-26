#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token, Address, BytesN, Env, IntoVal,
};

use crate::{
    core_contract::{Client as CoreContractClient, Dao, WASM as CoreWASM},
    types::{PropStatus, PROPOSAL_MAX_NR, RESERVE_AMOUNT, XLM},
    VotesContract, VotesContractClient,
};
use crate::types::Voting;

mod assets_contract {
    soroban_sdk::contractimport!(file = "../../wasm/elio_assets.wasm");
}

const PROPOSAL_DURATION: u32 = 10_000;
const MINT: i128 = 10_000 * XLM;

pub const MAX_I128: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;

struct Clients {
    core: CoreContractClient<'static>,
    votes: VotesContractClient<'static>,
    native_asset_admin: token::AdminClient<'static>,
}

impl Clients {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let core_id = env.register_contract_wasm(None, CoreWASM);
        let votes_id = env.register_contract(None, VotesContract);

        let core = CoreContractClient::new(&env, &core_id);
        let votes = VotesContractClient::new(&env, &votes_id);

        let native_asset_id = env.register_stellar_asset_contract(Address::random(&env));
        let native_asset_admin = token::AdminClient::new(&env, &native_asset_id);

        core.init(&votes_id, &native_asset_id);
        votes.init(&core_id);

        Self {
            core,
            votes,
            native_asset_admin,
        }
    }
}

fn create_dao(core: &CoreContractClient<'static>, dao_owner: &Address) -> Dao {
    let env = &core.env;

    let id = "DIV".into_val(env);
    let name = "Deep Ink Ventures".into_val(env);

    core.create_dao(&id, &name, &dao_owner)
}

fn mint_and_create_dao(clients: &Clients, dao_owner: &Address) -> Dao {
    clients.native_asset_admin.mint(&dao_owner, &MAX_I128);
    create_dao(&clients.core, &dao_owner)
}

fn mint_and_create_dao_with_asset(clients: &Clients, dao_owner: &Address) -> Dao {
    let dao = mint_and_create_dao(&clients, &dao_owner);

    let core = &clients.core;
    let env = &core.env;

    let assets_wasm_hash = env.deployer().upload_contract_wasm(assets_contract::WASM);
    let salt = BytesN::from_array(env, &[1; 32]);
    core.issue_token(&dao.id, &dao_owner, &assets_wasm_hash, &salt);

    dao
}

fn mint_and_create_dao_with_minted_asset(
    clients: &Clients,
    dao_owner: &Address,
    supply: i128,
) -> Dao {
    let dao = mint_and_create_dao_with_asset(clients, dao_owner);

    let core = &clients.core;
    let env = &core.env;

    let asset_id = core.get_dao_asset_id(&dao.id);
    let asset = assets_contract::Client::new(env, &asset_id);
    asset.mint(dao_owner, &supply);

    dao
}

fn create_dao_with_proposal(clients: &Clients, proposal_owner: &Address) -> (Dao, u32) {
    let Clients {
        core,
        votes,
        ..
    } = clients;
    let env = &core.env;

    let dao_owner = Address::random(env);
    let dao = mint_and_create_dao_with_minted_asset(&clients, &dao_owner, MAX_I128);

    let native_asset_id = core.get_native_asset_id();
    fund_account(&env, &native_asset_id,proposal_owner);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let proposal_id = votes.create_proposal(&dao.id, &proposal_owner);

    (dao, proposal_id)
}


fn setup_accepted_proposal(clients: &Clients) -> (u32, Address) {
    let (core, votes) = (&clients.core, &clients.votes);
    let env = &core.env;
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });
    let owner = Address::random(env);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    let assets_wasm_hash = env.deployer().upload_contract_wasm(assets_contract::WASM);
    let salt = BytesN::from_array(env, &[1; 32]);
    core.issue_token(&dao.id, &dao.owner, &assets_wasm_hash, &salt);

    let asset_id = core.get_dao_asset_id(&dao.id);
    let asset = assets_contract::Client::new(env, &asset_id);

    let supply = 1_000_000;
    asset.mint(&dao.owner, &supply);

    let voter = dao.owner.clone();
    votes.vote(&dao.id, &proposal_id, &true, &voter);

    // make finalization possible
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    votes.finalize_proposal(&dao.id, &proposal_id);

    let proposal = votes
        .get_active_proposals(&dao.id)
        .get_unchecked(0);
    assert_eq!(proposal.inner.status, PropStatus::Accepted);
    assert_eq!(proposal.inner, votes.get_archived_proposal(&proposal_id));
    (proposal_id, dao.owner)
}

fn fund_account(env: &Env, native_asset_id: &Address, address: &Address) {
    let native_token = token::AdminClient::new(&env, &native_asset_id);
    native_token.mint(&address, &MINT);
}

#[test]
fn active_proposals_are_managed() {
    let clients = Clients::new();
    let (core, votes) = (&clients.core, &clients.votes);
    let env = &core.env;

    env.budget().reset_unlimited();

    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    let dao_owner = Address::random(env);
    let dao = mint_and_create_dao_with_minted_asset(&clients, &dao_owner, MAX_I128);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit, &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let owner = Address::random(env);
    fund_account(&env, &core.get_native_asset_id(),&owner);
    let proposal_1_id = votes.create_proposal(&dao.id, &owner);

    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 200,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });
    let proposal_2_id = votes.create_proposal(&dao.id, &owner);

    let all_proposals = votes.get_active_proposals(&dao.id);
    assert_eq!(all_proposals.len(), 2);
    let p1 = all_proposals.get_unchecked(0);
    let p2 = all_proposals.get_unchecked(1);
    assert_eq!(p1.id, proposal_1_id);
    assert_eq!(p2.id, proposal_2_id);

    assert_eq!(p1.inner.ledger, 100);
    assert_eq!(p2.inner.ledger, 200);

    // outdate the first
    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + PROPOSAL_DURATION + 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    let all_proposals = votes.get_active_proposals(&dao.id);
    assert_eq!(all_proposals.len(), 1);
    let p = all_proposals.get_unchecked(0);
    assert_eq!(p.id, proposal_2_id);
}

#[test]
fn max_number_of_proposals() {
    let ref clients @ Clients { ref votes, ref core, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let dao_owner = Address::random(env);
    let dao = mint_and_create_dao_with_minted_asset(&clients, &dao_owner, MAX_I128);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let native_asset_id = &core.get_native_asset_id();

    for _ in 0..=(PROPOSAL_MAX_NR - 1) {
        let proposal_owner = &Address::random(&env);
        fund_account(&env, &native_asset_id, &proposal_owner);
        let _ = votes.create_proposal(&dao.id, proposal_owner);
    }
}

#[test]
#[should_panic(expected = "#2")]
fn error_on_max_number_of_proposals() {
    let ref clients @ Clients { ref votes, ref core, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let dao_owner = Address::random(env);
    let dao = mint_and_create_dao_with_minted_asset(&clients, &dao_owner, MAX_I128);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let native_asset_id = &core.get_native_asset_id();

    for _ in 0..=PROPOSAL_MAX_NR {
        let proposal_owner = &Address::random(&env);
        fund_account(&env, &native_asset_id, &proposal_owner);
        let _ = votes.create_proposal(&dao.id, proposal_owner);
    }
}

#[test]
fn set_metadata() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    let url = "https://deep-ink.ventures".into_val(env);
    let hash = "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(env);
    votes.set_metadata(&dao.id, &proposal_id, &url, &hash, &owner);

    let meta = votes.get_metadata(&proposal_id);
    assert_eq!(meta.url, url);
    assert_eq!(meta.hash, hash);
}

#[test]
#[should_panic(expected = "#7")]
fn set_metadata_only_owner() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    let url = "https://deep-ink.ventures".into_val(env);
    let hash = "e337ba02296d560d167b4c301505f1252c29bcf614893a806043d33fd3509181".into_val(env);
    let whoever = Address::random(env);
    votes.set_metadata(&dao.id, &proposal_id, &url, &hash, &whoever);
}

#[test]
#[should_panic(expected = "#8")]
fn non_existing_meta_panics() {
    let Clients { votes, .. } = Clients::new();

    votes.get_metadata(&0);
}

#[test]
fn set_configuration() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    let owner = Address::random(env);
    let dao = mint_and_create_dao(&clients, &owner);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let configuration = votes.get_configuration(&dao.id);
    assert_eq!(configuration.proposal_duration, proposal_duration);
    assert_eq!(configuration.proposal_token_deposit, proposal_token_deposit);
    assert_eq!(configuration.voting, voting);
}

#[test]
#[should_panic(expected = "#1")]
fn set_configuration_only_owner() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    let owner = Address::random(env);
    let dao = mint_and_create_dao(&clients, &owner);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    let whoever = Address::random(env);
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &whoever
    );
}

#[test]
#[should_panic(expected = "#1")]
fn remove_configuration_only_owner() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    let owner = Address::random(env);
    let dao = mint_and_create_dao(&clients, &owner);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    let whoever = Address::random(env);
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &owner
    );
    votes.remove_configuration(&dao.id, &whoever);
}

#[test]
#[should_panic(expected = "#9")]
fn non_existing_configuration_panics() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    let owner = Address::random(env);
    let dao = mint_and_create_dao(&clients, &owner);

    votes.get_configuration(&dao.id);
}

#[test]
#[should_panic(expected = "#9")]
fn must_create_configuration_before_proposal() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let dao = mint_and_create_dao_with_minted_asset(clients, &owner, MAX_I128);

    votes.create_proposal(&dao.id, &owner);
}

#[test]
fn mark_faulty() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    let reason = "bad".into_val(env);
    votes.fault_proposal(&dao.id, &proposal_id, &reason, &dao.owner);

    let proposal = votes
        .get_active_proposals(&dao.id)
        .get_unchecked(0);
    assert_eq!(proposal.inner.status, PropStatus::Faulty(reason));
}

#[test]
#[should_panic(expected = "#1")]
fn mark_faulty_only_owner() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    let reason = "bad".into_val(env);
    votes.fault_proposal(&dao.id, &proposal_id, &reason, &Address::random(env));
}

#[test]
fn vote() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let dao_owner = Address::random(env);
    let supply = 1_000_000;
    let dao = mint_and_create_dao_with_minted_asset(&clients, &dao_owner, supply);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let owner = Address::random(env);
    fund_account(&env, &clients.core.get_native_asset_id(),&owner);
    let proposal_id = votes.create_proposal(&dao.id, &owner);

    let voter = dao.owner;
    votes.vote(&dao.id, &proposal_id, &true, &voter);
    let proposal = votes
        .get_active_proposals(&dao.id)
        .get_unchecked(0);
    assert_eq!(proposal.in_favor, supply);
}

#[test]
fn rejected_finalize() {
    let ref clients @ Clients { ref votes, ..} = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });
    let owner = Address::random(env);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    let proposal_duration: u32 = 10_000;

    // make finalization possible
    votes.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + proposal_duration + 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    votes.finalize_proposal(&dao.id, &proposal_id);

    let proposal = votes.get_archived_proposal(&proposal_id);
    assert_eq!(proposal.status, PropStatus::Rejected);
}

#[test]
fn accepted_finalize() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    let dao_owner = Address::random(env);
    let supply = 1_000_000;
    let dao = mint_and_create_dao_with_minted_asset(&clients, &dao_owner, supply);

    let proposal_duration: u32 = 10_000;
    let proposal_token_deposit: u128 = 100_000_000;
    let min_threshold_configuration: i128 = 1_000;
    let voting = Voting::Majority;
    votes.set_configuration(
        &dao.id,
        &proposal_duration,
        &proposal_token_deposit,
        &min_threshold_configuration,
        &voting,
        &dao.owner
    );

    let proposal_id = votes.create_proposal(&dao.id, &dao_owner);

    votes.vote(&dao.id, &proposal_id, &true, &dao.owner);

    votes.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + proposal_duration + 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    votes.finalize_proposal(&dao.id, &proposal_id);

    let proposal = votes.get_archived_proposal(&proposal_id);
    assert_eq!(proposal.status, PropStatus::Accepted);
}

#[test]
#[ignore]
fn mark_implemented() {
    let clients = Clients::new();
    let (proposal_id, dao_owner) = setup_accepted_proposal(&clients);

    let votes = clients.votes;
    votes.mark_implemented(&proposal_id, &dao_owner);

    let proposal = votes.get_archived_proposal(&proposal_id);
    assert_eq!(proposal.status, PropStatus::Implemented);
}

#[test]
#[ignore]
#[should_panic(expected = "not the DAO owner")]
fn mark_implemented_only_owner() {
    let clients = Clients::new();
    let (proposal_id, _dao_owner) = setup_accepted_proposal(&clients);

    let votes = clients.votes;

    votes.mark_implemented(&proposal_id, &Address::random(&votes.env));
}

#[test]
fn reserves_token_on_proposal_creation() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let native_asset_id = &clients.core.get_native_asset_id();
    let native_token = token::Client::new(&env, &native_asset_id);

    create_dao_with_proposal(&clients, &owner);

    // Checks if balance deducted after proposal creation
    let current_balance = native_token.balance(&owner);
    assert_eq!(current_balance, &MINT - &RESERVE_AMOUNT);
}

#[test]
fn return_tokens_when_faulty() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    let owner = Address::random(env);
    let native_asset_id = &clients.core.get_native_asset_id();
    let native_token = token::Client::new(&env, &native_asset_id);
    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    // Checks if balance deducted after proposal creation
    let current_balance = native_token.balance(&owner);
    assert_eq!(current_balance, &MINT - &RESERVE_AMOUNT);

    let reason = "bad".into_val(env);
    votes.fault_proposal(&dao.id, &proposal_id, &reason, &dao.owner);

    // Check if reserved token has been returned
    let current_balance = native_token.balance(&owner);
    assert_eq!(&current_balance, &MINT)
}

#[test]
fn returns_tokens_on_finalize() {
    let ref clients @ Clients { ref votes, .. } = Clients::new();
    let env = &votes.env;

    env.budget().reset_unlimited();

    env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });
    let owner = Address::random(env);
    let native_asset_id = &clients.core.get_native_asset_id();
    let native_token = token::Client::new(&env, &native_asset_id);

    let (dao, proposal_id) = create_dao_with_proposal(&clients, &owner);

    // Checks if balance deducted after proposal creation
    let current_balance = native_token.balance(&owner);
    assert_eq!(current_balance, &MINT - &RESERVE_AMOUNT);

    let proposal_duration: u32 = 10_000;

    // make finalization possible
    votes.env.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number: 100 + proposal_duration + 1,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 10,
        min_persistent_entry_expiration: 10,
        max_entry_expiration: 10,
    });

    votes.finalize_proposal(&dao.id, &proposal_id);

    // Check if reserved token has been returned
    let current_balance = native_token.balance(&owner);
    assert_eq!(&current_balance, &MINT)
}