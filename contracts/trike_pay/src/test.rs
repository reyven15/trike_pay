#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _};
use soroban_sdk::{token, Address, Env};

#[test]
fn test_happy_path_payment() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let passenger = Address::generate(&e);
    let driver = Address::generate(&e);
    
    let token_admin = Address::generate(&e);
    let token = e.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&e, &token);
    token_client.mint(&passenger, &10000);

    let contract_id = e.register_contract(None, TrikePay);
    let client = TrikePayClient::new(&e, &contract_id);
    client.init(&admin);
    
    client.pay_ride(&passenger, &driver, &token, &1000);

    assert_eq!(token_client.balance(&driver), 999);
    assert_eq!(token_client.balance(&admin), 1);
}

#[test]
#[should_panic(expected = "Not initialized")]
fn test_fail_uninitialized() {
    let e = Env::default();
    e.mock_all_auths();
    let contract_id = e.register_contract(None, TrikePay);
    let client = TrikePayClient::new(&e, &contract_id);
    
    let passenger = Address::generate(&e);
    let driver = Address::generate(&e);
    let token = e.register_stellar_asset_contract(Address::generate(&e));

    client.pay_ride(&passenger, &driver, &token, &1000);
}

#[test]
fn test_state_verification() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let passenger = Address::generate(&e);
    let driver = Address::generate(&e);
    let token = e.register_stellar_asset_contract(Address::generate(&e));
    token::Client::new(&e, &token).mint(&passenger, &5000);

    let contract_id = e.register_contract(None, TrikePay);
    let client = TrikePayClient::new(&e, &contract_id);
    client.init(&admin);

    client.pay_ride(&passenger, &driver, &token, &2000);
    
    // Assert: Driver history in storage should be 1998 (2000 - 2 fee)
    assert_eq!(client.get_driver_total(&driver), 1998);
}

#[test]
#[should_panic]
fn test_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let passenger = Address::generate(&e);
    let driver = Address::generate(&e);
    let token = e.register_stellar_asset_contract(Address::generate(&e));
    // Mint 0 to passenger
    
    let contract_id = e.register_contract(None, TrikePay);
    let client = TrikePayClient::new(&e, &contract_id);
    client.init(&admin);

    client.pay_ride(&passenger, &driver, &token, &1000);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_prevent_double_init() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let contract_id = e.register_contract(None, TrikePay);
    let client = TrikePayClient::new(&e, &contract_id);
    
    client.init(&admin);
    client.init(&admin);
}