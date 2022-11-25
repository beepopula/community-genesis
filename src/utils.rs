

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
