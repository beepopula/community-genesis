

use ed25519_dalek::Verifier;
use near_sdk::{Balance, StorageUsage, Promise, log};

use crate::*;


pub(crate) fn refund_extra_storage_deposit(storage_used: StorageUsage, used_balance: Balance) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit()
        .checked_sub(used_balance)
        .expect("not enough attached balance");

    assert!(
        required_cost <= attached_deposit,
        "not enough attached balance {}",
        required_cost,
    );

    let refund = attached_deposit - required_cost;
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

pub(crate) fn get_env() -> String {
    let contract_id = env::current_account_id().to_string();
    let arr: Vec<String> = contract_id.split('.').map(|v| v.to_string()).collect();
    let env = arr.get(arr.len() - 1).unwrap();
    env.clone()
}

pub(crate) fn verify(message: &[u8], sign: &[u8], pk: &[u8]) -> bool {
    let pk = ed25519_dalek::PublicKey::from_bytes(&pk).unwrap();
    if sign.len() != 64 {
        panic!("Invalid signature data length.");
    }
    let mut sig_data: [u8; 64] = [0; 64];
    for i in 0..64 {
        sig_data[i] = sign.get(i).unwrap_or(&0).clone();
    }
    let sign = ed25519_dalek::Signature::try_from(sig_data).unwrap();
    match pk.verify(&message, &sign) {
        Ok(_) => true,
        Err(_) => false,
    }
}