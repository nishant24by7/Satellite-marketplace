#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map, String};

#[contract]
pub struct SatelliteMarketplace;

#[contractimpl]
impl SatelliteMarketplace {

    // Storage Keys
    fn listings_key(env: &Env) -> Symbol {
        symbol_short!("LISTINGS")
    }

    // Add a satellite listing
    pub fn list_satellite(
        env: Env,
        seller: Address,
        satellite_id: String,
        price: i128,
        description: String,
    ) {
        seller.require_auth();

        let mut listings: Map<String, (Address, i128, String)> =
            env.storage().instance().get(&Self::listings_key(&env)).unwrap_or(Map::new(&env));

        listings.set(satellite_id.clone(), (seller, price, description));

        env.storage().instance().set(&Self::listings_key(&env), &listings);
    }

    // Buy satellite
    pub fn buy_satellite(
        env: Env,
        buyer: Address,
        satellite_id: String,
    ) {
        buyer.require_auth();

        let mut listings: Map<String, (Address, i128, String)> =
            env.storage().instance().get(&Self::listings_key(&env)).unwrap();

        let (seller, price, _desc) = listings.get(satellite_id.clone()).unwrap();

        // Transfer logic would go here (token transfer not implemented yet)

        listings.remove(satellite_id);

        env.storage().instance().set(&Self::listings_key(&env), &listings);
    }

    // View listing
    pub fn get_listing(env: Env, satellite_id: String) -> Option<(Address, i128, String)> {
        let listings: Map<String, (Address, i128, String)> =
            env.storage().instance().get(&Self::listings_key(&env)).unwrap_or(Map::new(&env));

        listings.get(satellite_id)
    }
}
