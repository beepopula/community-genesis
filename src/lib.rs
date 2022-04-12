

use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::thread::AccessError;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BorshStorageKey};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};


pub mod utils;
pub mod resolver;
pub mod view;
pub mod owner;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CommunityGenesis {
    owner_id: AccountId,
    communities: UnorderedMap<AccountId, Community>,
    codes: UnorderedMap<String, CodeInfo>,
    accounts: UnorderedMap<AccountId, Vec<AccountId>>,
    public_key: String,
    account_storage_usage: u128
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Community {
    contract_id: AccountId,
    owner_id: AccountId,
    community_type: String,
    code_hash: Base58CryptoHash
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct CodeInfo {
    hash: Base58CryptoHash,
    length: u32,
    storage_deposit: U128
}

#[derive(BorshSerialize, BorshStorageKey)]
#[derive(Debug)]
pub enum StorageKey{
    Communities,
    Codes,
    Accounts
}

#[near_bindgen]
impl CommunityGenesis {

    #[init]
    pub fn new(public_key: String) -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            communities: UnorderedMap::new(StorageKey::Communities),
            codes: UnorderedMap::new(StorageKey::Codes),
            accounts: UnorderedMap::new(StorageKey::Accounts),
            public_key,
            account_storage_usage: 128
        }
    }

    #[payable]
    pub fn deploy_community(&mut self, name: String, community_type: String, args: Option<String>) {
        let sender_id = env::predecessor_account_id();
        let code_info = self.codes.get(&community_type).unwrap();
        let contract_id: AccountId = AccountId::from_str(&(name + "." + &env::current_account_id().to_string())).unwrap();

        Promise::new(contract_id.clone())
        .create_account()
        .transfer(u128::from(code_info.storage_deposit) + self.account_storage_usage as u128 * env::storage_byte_cost())
        .deploy_contract(env::storage_read(community_type.as_bytes()).unwrap())
        .function_call("new".into(), json!({
            "owner_id": sender_id,
            "public_key": self.public_key,
            "args": args 
        }).to_string().as_bytes().to_vec(), 0, (env::prepaid_gas() - env::used_gas()) / 3).then(
            Promise::new(env::current_account_id())
            .function_call("on_add_community".into(), json!({
                "contract_id": contract_id,
                "community_type": community_type,
                "owner_id": sender_id,
                "code_hash": code_info.hash
            }).to_string().into(), env::attached_deposit(), (env::prepaid_gas() - env::used_gas()) / 3)
        );
    }

    #[payable]
    pub fn update_community(&mut self, contract_id: AccountId, community_type: String, args:Option<String>) {
        let sender_id = env::predecessor_account_id();
        let community = self.communities.get(&contract_id).unwrap();
        assert!(sender_id == community.owner_id, "not owner");
        let promise = Promise::new(contract_id.clone())
        .deploy_contract(env::storage_read(community_type.as_bytes()).unwrap());
        let promise = match args {
            Some(v) => {
                promise.function_call("migrate".to_string(), v.into_bytes(), 0, (env::prepaid_gas() - env::used_gas()) / 3)
            },
            None => promise
        };
        promise.then(
            Promise::new(env::current_account_id()).function_call("on_update_community".to_string(), json!({
                "contract_id": contract_id,
                "community_type": community_type
            }).to_string().as_bytes().to_vec(), env::attached_deposit(), (env::prepaid_gas() - env::used_gas()) / 3)
        );
    }
}



#[cfg(test)]
mod tests {


}