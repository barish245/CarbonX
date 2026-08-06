#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, IntoVal, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    pub id: u64,
    pub owner: Address,
    pub credit_id: u64,
    pub amount: u128,
}

#[contract]
pub struct RetirementContract;

#[contractimpl]
impl RetirementContract {
    pub fn retire_credit(env: Env, owner: Address, registry_contract: Address, credit_id: u64) -> u64 {
        owner.require_auth();

        let r_client = RegistryClient::new(&env, &registry_contract);
        let amount = r_client.get_credit_amount(&credit_id);

        r_client.retire(&owner, &credit_id);

        let mut cert_count: u64 = env.storage().instance().get(&symbol_short!("certs")).unwrap_or(0);
        cert_count += 1;
        env.storage().instance().set(&symbol_short!("certs"), &cert_count);

        let cert = Certificate {
            id: cert_count,
            owner: owner.clone(),
            credit_id,
            amount,
        };
        env.storage().instance().set(&cert_count, &cert);

        let key_score = (symbol_short!("score"), owner.clone());
        let current_score: u32 = env.storage().instance().get(&key_score).unwrap_or(50);
        let score_delta = (amount * 10) as u32;
        let new_score = (current_score + score_delta).min(100);
        env.storage().instance().set(&key_score, &new_score);

        env.events().publish(
            (symbol_short!("retired"), credit_id),
            (owner.clone(), amount),
        );

        env.events().publish(
            (symbol_short!("cert_iss"), cert_count),
            (owner, new_score),
        );

        cert_count
    }

    pub fn batch_retire(env: Env, owner: Address, registry_contract: Address, credit_ids: Vec<u64>) -> Vec<u64> {
        owner.require_auth();
        let mut issued_certs: Vec<u64> = Vec::new(&env);
        for id in credit_ids.iter() {
            let cert_id = Self::retire_credit(env.clone(), owner.clone(), registry_contract.clone(), id);
            issued_certs.push_back(cert_id);
        }
        issued_certs
    }

    pub fn get_certificate(env: Env, cert_id: u64) -> Certificate {
        env.storage().instance().get(&cert_id).expect("certificate not found")
    }

    pub fn get_cert_count(env: Env) -> u64 {
        env.storage().instance().get(&symbol_short!("certs")).unwrap_or(0)
    }

    pub fn get_score(env: Env, address: Address) -> u32 {
        let key_score = (symbol_short!("score"), address);
        env.storage().instance().get(&key_score).unwrap_or(50)
    }

    pub fn get_carbon_tier(env: Env, address: Address) -> Symbol {
        let score = Self::get_score(env.clone(), address);
        if score >= 90 {
            Symbol::new(&env, "Platinum")
        } else if score >= 75 {
            Symbol::new(&env, "Gold")
        } else if score >= 60 {
            Symbol::new(&env, "Silver")
        } else {
            Symbol::new(&env, "Bronze")
        }
    }
}

struct RegistryClient<'a> {
    env: &'a Env,
    address: &'a Address,
}

impl<'a> RegistryClient<'a> {
    fn new(env: &'a Env, address: &'a Address) -> Self {
        Self { env, address }
    }
    
    fn get_credit_amount(&self, credit_id: &u64) -> u128 {
        let credit: Credit = self.env.invoke_contract(
            self.address,
            &Symbol::new(self.env, "get_credit"),
            soroban_sdk::vec![self.env, credit_id.clone().into_val(self.env)],
        );
        credit.amount
    }

    fn retire(&self, owner: &Address, credit_id: &u64) {
        self.env.invoke_contract::<()>(
            self.address,
            &Symbol::new(self.env, "retire"),
            soroban_sdk::vec![self.env, owner.clone().into_val(self.env), credit_id.clone().into_val(self.env)],
        );
    }
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credit {
    pub id: u64,
    pub original_owner: Address,
    pub amount: u128,
    pub project_id: u64,
    pub retired: bool,
}
