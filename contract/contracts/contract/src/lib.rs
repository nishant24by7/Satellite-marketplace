#![no_std]
use soroban_sdk::{
    contract, contractimpl, Address, Env
};

#[contract]
pub struct SimplePayment;

#[contractimpl]
impl SimplePayment {

    // Function to send XLM from sender to receiver
    pub fn send_payment(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        token_address: Address, // Native XLM contract address
    ) {
        // Require sender authorization
        from.require_auth();

        // Create token client (XLM contract)
        let token_client = soroban_sdk::token::Client::new(&env, &token_address);

        // Transfer XLM
        token_client.transfer(&from, &to, &amount);

        // Emit event
        env.events().publish(
            ("payment", &from, &to),
            amount
        );
    }
}