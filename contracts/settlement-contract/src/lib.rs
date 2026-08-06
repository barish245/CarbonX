#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct SettlementContract;

#[contractimpl]
impl SettlementContract {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&symbol_short!("admin")) {
            panic!("already initialized");
        }
        env.storage().instance().set(&symbol_short!("admin"), &admin);
    }

    pub fn settle_payment(env: Env, buyer: Address, seller: Address, amount: u128) -> u64 {
        buyer.require_auth();
        if amount == 0 {
            panic!("amount must be greater than zero");
        }

        let mut pay_count: u64 = env.storage().instance().get(&symbol_short!("count")).unwrap_or(0);
        pay_count += 1;
        env.storage().instance().set(&symbol_short!("count"), &pay_count);

        env.events().publish(
            (symbol_short!("pay_lock"), buyer.clone()),
            (pay_count, amount),
        );

        env.events().publish(
            (symbol_short!("pay_rel"), seller.clone()),
            (pay_count, amount),
        );

        pay_count
    }

    pub fn dispute_payment(env: Env, buyer: Address, payment_id: u64, reason: Symbol) {
        buyer.require_auth();
        env.events().publish(
            (symbol_short!("disputed"), payment_id),
            (buyer, reason),
        );
    }

    pub fn resolve_dispute(env: Env, payment_id: u64, refund_buyer: bool) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).expect("not initialized");
        admin.require_auth();

        env.events().publish(
            (symbol_short!("resolved"), payment_id),
            refund_buyer,
        );
    }

    pub fn get_payment_count(env: Env) -> u64 {
        env.storage().instance().get(&symbol_short!("count")).unwrap_or(0)
    }
}
