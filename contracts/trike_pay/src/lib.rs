
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    DriverBalance(Address), // Stores total earnings per driver
    Admin,                  // Address that receives the platform fee
}

#[contract]
pub struct TrikePay;

#[contractimpl]
impl TrikePay {
    // Initializes the contract by setting the fee collector (Admin)
    pub fn init(e: Env, admin: Address) {
        if e.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        e.storage().instance().set(&DataKey::Admin, &admin);
    }

    // MVP Core Feature: Passenger pays driver, platform takes 0.1% fee
    pub fn pay_ride(e: Env, passenger: Address, driver: Address, token_addr: Address, amount: i128) {
        // 1. Verify passenger identity
        passenger.require_auth();

        // 2. Fetch admin address for fee collection
        let admin: Address = e.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        let token_client = token::Client::new(&e, &token_addr);

        // 3. Calculate 0.1% fee (amount / 1000)
        let fee = amount / 1000;
        let driver_payout = amount - fee;

        // 4. Perform on-chain transfers
        token_client.transfer(&passenger, &admin, &fee);
        token_client.transfer(&passenger, &driver, &driver_payout);

        // 5. Update driver's on-chain earnings history
        let current_total: i128 = e.storage().instance().get(&DataKey::DriverBalance(driver.clone())).unwrap_or(0);
        e.storage().instance().set(&DataKey::DriverBalance(driver), &(current_total + driver_payout));
    }

    // Helper function to verify driver total earnings on-chain
    pub fn get_driver_total(e: Env, driver: Address) -> i128 {
        e.storage().instance().get(&DataKey::DriverBalance(driver)).unwrap_or(0)
    }
}