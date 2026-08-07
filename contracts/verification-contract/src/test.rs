#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, Symbol, Vec};
use soroban_sdk::testutils::Address as _;
use carbon_registry::{CarbonRegistry, CarbonRegistryClient};
use marketplace_contract::{MarketplaceContract, MarketplaceContractClient};
use settlement_contract::{SettlementContract, SettlementContractClient};
use retirement_contract::{RetirementContract, RetirementContractClient};

#[test]
fn test_full_carbon_marketplace_flow() {
    let env = Env::default();
    env.mock_all_auths();

    // 1. Generate addresses
    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);
    let developer = Address::generate(&env);
    let buyer = Address::generate(&env);

    // 2. Register and initialize Verification Contract
    let verification_id = env.register_contract(None, VerificationContract);
    let verification_client = VerificationContractClient::new(&env, &verification_id);
    verification_client.init(&admin);
    verification_client.add_verifier(&verifier);

    // 3. Register and initialize Carbon Registry
    let registry_id = env.register_contract(None, CarbonRegistry);
    let registry_client = CarbonRegistryClient::new(&env, &registry_id);
    registry_client.init(&verification_id);

    // 4. Register and initialize Marketplace Contract
    let marketplace_id = env.register_contract(None, MarketplaceContract);
    let marketplace_client = MarketplaceContractClient::new(&env, &marketplace_id);
    marketplace_client.init(&admin);

    // 5. Register and initialize Settlement Contract
    let settlement_id = env.register_contract(None, SettlementContract);
    let settlement_client = SettlementContractClient::new(&env, &settlement_id);
    settlement_client.init(&admin);

    // 6. Register Retirement Contract
    let retirement_id = env.register_contract(None, RetirementContract);
    let retirement_client = RetirementContractClient::new(&env, &retirement_id);

    // Assert initial state
    assert!(verification_client.is_verifier(&verifier));
    assert!(!verification_client.is_verifier(&developer));
    assert_eq!(verification_client.get_project_count(), 0);

    // --- SUBMISSION & MINTING ---
    let desc = Symbol::new(&env, "Solar_Plant_Kolkata");
    let project_id = verification_client.submit_project(&developer, &desc, &120);
    assert_eq!(project_id, 1);
    assert_eq!(verification_client.get_project_count(), 1);

    // Verify project, which invokes registry to mint 120 credits to developer
    verification_client.verify_project(&verifier, &project_id, &registry_id);
    assert_eq!(verification_client.get_verifier_reputation(&verifier), 1);

    // Assert credits are minted to developer
    assert_eq!(registry_client.get_balance(&developer), 120);
    assert_eq!(registry_client.get_owner(&1), developer.clone());
    assert_eq!(registry_client.get_total_minted(), 120);

    // --- MARKETPLACE LISTING ---
    // Developer lists the minted credit ID 1 for 50 XLM
    let listing_id = marketplace_client.list_credit(&developer, &registry_id, &1, &50);
    assert_eq!(listing_id, 1);
    assert_eq!(marketplace_client.get_listing_count(), 1);

    // --- PURCHASE & SETTLEMENT ---
    // Buyer purchases the listing
    marketplace_client.buy_credit(&buyer, &listing_id, &registry_id, &settlement_id);

    // Assert ownership has transferred to the buyer
    assert_eq!(registry_client.get_balance(&developer), 0);
    assert_eq!(registry_client.get_balance(&buyer), 120);
    assert_eq!(registry_client.get_owner(&1), buyer.clone());

    // --- RETIREMENT & CARBON SCORE ---
    // Buyer retires the carbon credits
    let cert_id = retirement_client.retire_credit(&buyer, &registry_id, &1);
    assert_eq!(cert_id, 1);

    // Assert credits are retired
    assert_eq!(registry_client.get_balance(&buyer), 0);
    assert_eq!(registry_client.get_retired_balance(&buyer), 120);
    assert_eq!(registry_client.get_total_retired(), 120);

    // Assert Carbon Impact Score has increased (base score 50 + 120*10 = 1250, capped at 100)
    let final_score = retirement_client.get_score(&buyer);
    assert_eq!(final_score, 100);

    let tier = retirement_client.get_carbon_tier(&buyer);
    assert_eq!(tier, Symbol::new(&env, "Platinum"));

    let cert = retirement_client.get_certificate(&1);
    assert_eq!(cert.amount, 120);
    assert_eq!(cert.owner, buyer);
}

#[test]
fn test_verifier_management_and_penalties() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);

    let verification_id = env.register_contract(None, VerificationContract);
    let client = VerificationContractClient::new(&env, &verification_id);
    client.init(&admin);

    client.add_verifier(&verifier);
    assert!(client.is_verifier(&verifier));

    // Remove verifier
    client.remove_verifier(&verifier);
    assert!(!client.is_verifier(&verifier));

    // Re-add and test penalty
    client.add_verifier(&verifier);
    client.penalize_verifier(&verifier, &5);
    assert_eq!(client.get_verifier_reputation(&verifier), 0);
}

#[test]
fn test_marketplace_pause_and_fees() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let marketplace_id = env.register_contract(None, MarketplaceContract);
    let client = MarketplaceContractClient::new(&env, &marketplace_id);
    client.init(&admin);

    assert!(!client.is_paused());
    assert_eq!(client.get_fee_bps(), 100);

    client.set_fee_bps(&250);
    assert_eq!(client.get_fee_bps(), 250);

    client.pause();
    assert!(client.is_paused());

    client.unpause();
    assert!(!client.is_paused());
}

#[test]
fn test_batch_operations() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let verifier = Address::generate(&env);
    let developer = Address::generate(&env);
    let recipient = Address::generate(&env);

    let verification_id = env.register_contract(None, VerificationContract);
    let verification_client = VerificationContractClient::new(&env, &verification_id);
    verification_client.init(&admin);
    verification_client.add_verifier(&verifier);

    let registry_id = env.register_contract(None, CarbonRegistry);
    let registry_client = CarbonRegistryClient::new(&env, &registry_id);
    registry_client.init(&verification_id);

    // Submit and verify two projects
    let p1 = verification_client.submit_project(&developer, &Symbol::new(&env, "P1"), &50);
    let p2 = verification_client.submit_project(&developer, &Symbol::new(&env, "P2"), &75);

    verification_client.verify_project(&verifier, &p1, &registry_id);
    verification_client.verify_project(&verifier, &p2, &registry_id);

    assert_eq!(registry_client.get_balance(&developer), 125);

    // Batch transfer
    let mut credit_ids: Vec<u64> = Vec::new(&env);
    credit_ids.push_back(1);
    credit_ids.push_back(2);

    registry_client.batch_transfer(&developer, &recipient, &credit_ids);

    assert_eq!(registry_client.get_balance(&developer), 0);
    assert_eq!(registry_client.get_balance(&recipient), 125);
}
