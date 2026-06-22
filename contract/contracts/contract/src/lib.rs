#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Symbol, Vec};

#[contracttype]
pub enum DataKey {
    Admin,
    Candidates,
    Votes,
    Voters,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn init(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Candidates, &Vec::<String>::new(&env));
        env.storage().instance().set(&DataKey::Votes, &Map::<String, u32>::new(&env));
        env.storage().instance().set(&DataKey::Voters, &Map::<Address, bool>::new(&env));
    }

    pub fn add_candidate(env: Env, caller: Address, name: String) {
        caller.require_auth();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert_eq!(caller, admin, "only admin can add candidates");
        let mut candidates: Vec<String> = env.storage().instance().get(&DataKey::Candidates).unwrap();
        candidates.push_back(name.clone());
        env.storage().instance().set(&DataKey::Candidates, &candidates);
        let mut votes: Map<String, u32> = env.storage().instance().get(&DataKey::Votes).unwrap();
        votes.set(name.clone(), 0);
        env.storage().instance().set(&DataKey::Votes, &votes);
        env.events().publish((Symbol::new(&env, "candidate_added"),), (name, 0u32));
    }

    pub fn vote(env: Env, voter: Address, candidate: String) {
        voter.require_auth();
        let mut voters: Map<Address, bool> = env.storage().instance().get(&DataKey::Voters).unwrap();
        assert!(!voters.get(voter.clone()).unwrap_or(false), "already voted");
        let mut votes: Map<String, u32> = env.storage().instance().get(&DataKey::Votes).unwrap();
        let count = votes.get(candidate.clone()).unwrap_or(0);
        votes.set(candidate.clone(), count + 1);
        voters.set(voter.clone(), true);
        env.storage().instance().set(&DataKey::Votes, &votes);
        env.storage().instance().set(&DataKey::Voters, &voters);
        env.events().publish((Symbol::new(&env, "vote"),), (voter, candidate, count + 1));
    }

    pub fn get_votes(env: Env, candidate: String) -> u32 {
        let votes: Map<String, u32> = env.storage().instance().get(&DataKey::Votes).unwrap();
        votes.get(candidate).unwrap_or(0)
    }

    pub fn get_candidates(env: Env) -> Vec<String> {
        env.storage().instance().get(&DataKey::Candidates).unwrap()
    }

    pub fn has_voted(env: Env, voter: Address) -> bool {
        let voters: Map<Address, bool> = env.storage().instance().get(&DataKey::Voters).unwrap();
        voters.get(voter).unwrap_or(false)
    }

    pub fn get_total_votes(env: Env) -> u32 {
        let votes: Map<String, u32> = env.storage().instance().get(&DataKey::Votes).unwrap();
        votes.values().iter().sum()
    }
}

mod test;
