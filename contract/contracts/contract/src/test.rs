#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, vec, Env, String};

#[test]
fn test_init_and_add_candidates() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init(&admin);

    let candidates = client.get_candidates();
    assert_eq!(candidates.len(), 0);

    client.add_candidate(&admin, &String::from_str(&env, "Alice"));
    client.add_candidate(&admin, &String::from_str(&env, "Bob"));
    client.add_candidate(&admin, &String::from_str(&env, "Charlie"));

    let candidates = client.get_candidates();
    assert_eq!(candidates.len(), 3);
    assert_eq!(candidates.get(0).unwrap(), String::from_str(&env, "Alice"));
    assert_eq!(candidates.get(1).unwrap(), String::from_str(&env, "Bob"));
    assert_eq!(candidates.get(2).unwrap(), String::from_str(&env, "Charlie"));
}

#[test]
fn test_vote_and_get_votes() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);

    client.init(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));
    client.add_candidate(&admin, &String::from_str(&env, "Bob"));

    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 0);
    assert_eq!(client.get_votes(&String::from_str(&env, "Bob")), 0);
    assert_eq!(client.get_total_votes(), 0);

    client.vote(&voter1, &String::from_str(&env, "Alice"));
    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 1);
    assert_eq!(client.get_total_votes(), 1);

    client.vote(&voter2, &String::from_str(&env, "Alice"));
    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 2);
    assert_eq!(client.get_total_votes(), 2);

    client.vote(&Address::generate(&env), &String::from_str(&env, "Bob"));
    assert_eq!(client.get_votes(&String::from_str(&env, "Bob")), 1);
    assert_eq!(client.get_total_votes(), 3);
}

#[test]
fn test_has_voted() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);

    client.init(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));

    assert!(!client.has_voted(&voter));
    client.vote(&voter, &String::from_str(&env, "Alice"));
    assert!(client.has_voted(&voter));
}

#[test]
#[should_panic(expected = "already voted")]
fn test_double_vote_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);

    client.init(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));
    client.vote(&voter, &String::from_str(&env, "Alice"));
    client.vote(&voter, &String::from_str(&env, "Alice"));
}

#[test]
#[should_panic(expected = "only admin can add candidates")]
fn test_non_admin_cannot_add_candidate() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);

    client.init(&admin);
    client.add_candidate(&non_admin, &String::from_str(&env, "Alice"));
}

#[test]
fn test_vote_for_nonexistent_candidate() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);

    client.init(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));

    assert_eq!(client.get_votes(&String::from_str(&env, "Nonexistent")), 0);
}

#[test]
fn test_multiple_voters_different_candidates() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voters: Vec<Address> = (0..5).map(|_| Address::generate(&env)).collect();

    client.init(&admin);
    client.add_candidate(&admin, &String::from_str(&env, "Alice"));
    client.add_candidate(&admin, &String::from_str(&env, "Bob"));

    for i in 0..5 {
        let candidate = if i % 2 == 0 { "Alice" } else { "Bob" };
        client.vote(&voters.get(i).unwrap(), &String::from_str(&env, candidate));
    }

    assert_eq!(client.get_votes(&String::from_str(&env, "Alice")), 3);
    assert_eq!(client.get_votes(&String::from_str(&env, "Bob")), 2);
    assert_eq!(client.get_total_votes(), 5);
}

#[test]
fn test_events_emitted() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);

    client.init(&admin);

    // Reset event buffer after init
    let _ = env.events().all();

    client.add_candidate(&admin, &String::from_str(&env, "Alice"));
    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(last.topics().get(0).unwrap(), Symbol::new(&env, "candidate_added"));

    client.vote(&voter, &String::from_str(&env, "Alice"));
    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(last.topics().get(0).unwrap(), Symbol::new(&env, "vote"));
}
