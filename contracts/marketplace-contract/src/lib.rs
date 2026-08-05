#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, IntoVal};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Listing {
    pub id: u64,
    pub seller: Address,
    pub credit_id: u64,
    pub price: u128,
    pub active: bool,
}

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&symbol_short!("admin")) {
            panic!("already initialized");
        }
        env.storage().instance().set(&symbol_short!("admin"), &admin);
        env.storage().instance().set(&symbol_short!("fee_bps"), &100u32); // 1% default fee (100 bps)
        env.storage().instance().set(&symbol_short!("paused"), &false);
    }

    pub fn set_fee_bps(env: Env, fee_bps: u32) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).expect("not initialized");
        admin.require_auth();
        if fee_bps > 1000 {
            panic!("fee cannot exceed 10%");
        }
        env.storage().instance().set(&symbol_short!("fee_bps"), &fee_bps);
    }

    pub fn get_fee_bps(env: Env) -> u32 {
        env.storage().instance().get(&symbol_short!("fee_bps")).unwrap_or(100)
    }

    pub fn pause(env: Env) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).expect("not initialized");
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("paused"), &true);
    }

    pub fn unpause(env: Env) {
        let admin: Address = env.storage().instance().get(&symbol_short!("admin")).expect("not initialized");
        admin.require_auth();
        env.storage().instance().set(&symbol_short!("paused"), &false);
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage().instance().get(&symbol_short!("paused")).unwrap_or(false)
    }

    pub fn list_credit(env: Env, seller: Address, registry_contract: Address, credit_id: u64, price: u128) -> u64 {
        seller.require_auth();
        if Self::is_paused(env.clone()) {
            panic!("marketplace is paused");
        }
        if price == 0 {
            panic!("price must be greater than zero");
        }
        
        let r_client = RegistryClient::new(&env, &registry_contract);
        r_client.transfer(&seller, &env.current_contract_address(), &credit_id);

        let mut listing_count: u64 = env.storage().instance().get(&symbol_short!("count")).unwrap_or(0);
        listing_count += 1;
        env.storage().instance().set(&symbol_short!("count"), &listing_count);

        let listing = Listing {
            id: listing_count,
            seller,
            credit_id,
            price,
            active: true,
        };

        env.storage().instance().set(&listing_count, &listing);

        env.events().publish(
            (symbol_short!("listed"), listing_count),
            (credit_id, price),
        );

        listing_count
    }

    pub fn cancel_listing(env: Env, seller: Address, listing_id: u64, registry_contract: Address) {
        seller.require_auth();
        let mut listing: Listing = env.storage().instance().get(&listing_id).expect("listing not found");
        if listing.seller != seller {
            panic!("not the seller");
        }
        if !listing.active {
            panic!("listing not active");
        }
        listing.active = false;
        env.storage().instance().set(&listing_id, &listing);

        let r_client = RegistryClient::new(&env, &registry_contract);
        r_client.transfer(&env.current_contract_address(), &listing.seller, &listing.credit_id);

        env.events().publish(
            (symbol_short!("cancelled"), listing_id),
            listing.credit_id,
        );
    }

    pub fn buy_credit(env: Env, buyer: Address, listing_id: u64, registry_contract: Address, settlement_contract: Address) {
        buyer.require_auth();
        if Self::is_paused(env.clone()) {
            panic!("marketplace is paused");
        }
        let mut listing: Listing = env.storage().instance().get(&listing_id).expect("listing not found");
        if !listing.active {
            panic!("listing not active");
        }
        listing.active = false;
        env.storage().instance().set(&listing_id, &listing);

        let s_client = SettlementClient::new(&env, &settlement_contract);
        s_client.settle_payment(&buyer, &listing.seller, &listing.price);

        let r_client = RegistryClient::new(&env, &registry_contract);
        r_client.transfer(&env.current_contract_address(), &buyer, &listing.credit_id);

        env.events().publish(
            (symbol_short!("purchased"), listing_id),
            (buyer, listing.credit_id),
        );
    }

    pub fn get_listing(env: Env, listing_id: u64) -> Listing {
        env.storage().instance().get(&listing_id).expect("listing not found")
    }

    pub fn get_listing_count(env: Env) -> u64 {
        env.storage().instance().get(&symbol_short!("count")).unwrap_or(0)
    }
}

struct SettlementClient<'a> {
    env: &'a Env,
    address: &'a Address,
}

impl<'a> SettlementClient<'a> {
    fn new(env: &'a Env, address: &'a Address) -> Self {
        Self { env, address }
    }
    fn settle_payment(&self, buyer: &Address, seller: &Address, amount: &u128) {
        let _: u64 = self.env.invoke_contract(
            self.address,
            &Symbol::new(self.env, "settle_payment"),
            soroban_sdk::vec![self.env, buyer.clone().into_val(self.env), seller.clone().into_val(self.env), amount.clone().into_val(self.env)],
        );
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
    fn transfer(&self, from: &Address, to: &Address, credit_id: &u64) {
        self.env.invoke_contract::<()>(
            self.address,
            &Symbol::new(self.env, "transfer"),
            soroban_sdk::vec![self.env, from.clone().into_val(self.env), to.clone().into_val(self.env), credit_id.clone().into_val(self.env)],
        );
    }
}
