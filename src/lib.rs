

use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::hash::Hash;
use std::str::FromStr;
use std::thread::AccessError;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::de::IntoDeserializer;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BorshStorageKey, CryptoHash, PublicKey};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};

use crate::utils::{refund_extra_storage_deposit, get_env, verify};


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
    args: HashMap<String, String>,
    account_storage_usage: u128,
    options: HashMap<String, String>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldCommunityGenesis {
    owner_id: AccountId,
    communities: UnorderedMap<AccountId, Community>,
    codes: UnorderedMap<String, CodeInfo>,
    accounts: UnorderedMap<AccountId, Vec<AccountId>>,
    args: HashMap<String, String>,
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

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct OldCodeInfo {
    hash: String,
    length: u32,
    storage_deposit: U128
}

#[derive(BorshSerialize, BorshStorageKey)]
#[derive(Debug)]
pub enum StorageKey{
    Communities,
    Codes,
    NewCodes,
    Accounts
}

const EXTRA_STORAGE_COST: u128 = 2_000_000_000_000_000_000_000_000;

#[near_bindgen]
impl CommunityGenesis {

    #[init]
    pub fn new(args: HashMap<String, String>, options: HashMap<String, String>) -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            communities: UnorderedMap::new(StorageKey::Communities),
            codes: UnorderedMap::new(StorageKey::Codes),
            accounts: UnorderedMap::new(StorageKey::Accounts),
            args,
            options,
            account_storage_usage: 128
        }
    }

    #[init(ignore_state)]
    pub fn migrate(options: HashMap<String, String>) -> Self {
        let old_contract: OldCommunityGenesis = env::state_read().unwrap();
        assert!(old_contract.owner_id == env::predecessor_account_id(), "owner only");
        let new_contract = CommunityGenesis {
            codes: UnorderedMap::new(StorageKey::NewCodes),
            owner_id: old_contract.owner_id,
            communities: old_contract.communities,
            accounts: old_contract.accounts,
            args: old_contract.args,
            options,
            account_storage_usage: old_contract.account_storage_usage
        };
        new_contract
    }

    #[payable]
    pub fn deploy_community(&mut self, name: String, community_type: String, options: Option<HashMap<String, String>>) {
        if get_env() == "testnet" {
            let options = options.clone().expect("not allowed");
            let nonce = options.get("nonce").unwrap();
            assert!(env::storage_has_key(nonce.as_bytes()) == false, "code already used");
            let sign = options.get("sign").unwrap();
            let sign = bs58::decode(sign).into_vec().unwrap();
            let pk = bs58::decode(self.options.get("invite_pk").unwrap()).into_vec().unwrap();
            assert!(verify(nonce.as_bytes(), &sign, &pk), "invalid code");
        }
        let sender_id = env::predecessor_account_id();
        let code_info = self.codes.get(&community_type).unwrap();
        let contract_id: AccountId = AccountId::from_str(&(name + "." + &env::current_account_id().to_string())).unwrap();
        let hash: Vec<u8> = CryptoHash::from(code_info.hash).to_vec();
        let storage_cost = self.account_storage_usage * env::storage_byte_cost() + u128::from(code_info.storage_deposit) + EXTRA_STORAGE_COST;

        assert!(env::attached_deposit() > storage_cost, "not enough deposit");

        Promise::new(contract_id.clone())
        .create_account()
        .add_full_access_key(PublicKey::from_str(&self.args.get("public_key").unwrap()).unwrap())
        .transfer(u128::from(code_info.storage_deposit) + EXTRA_STORAGE_COST)
        .deploy_contract(env::storage_read(&hash).unwrap())
        .function_call("new".into(), json!({
            "owner_id": sender_id,
            "args": json!(self.args) 
        }).to_string().as_bytes().to_vec(), 0, (env::prepaid_gas() - env::used_gas()) / 3).then(
            Promise::new(env::current_account_id())
            .function_call("on_add_community".into(), json!({
                "contract_id": contract_id,
                "community_type": community_type,
                "owner_id": sender_id,
                "options": options
            }).to_string().into(), 0, (env::prepaid_gas() - env::used_gas()) / 3)
        );
    }

    #[payable]
    pub fn update_community(&mut self, contract_id: AccountId, community_type: String, migrate: bool) {
        // let sender_id = env::predecessor_account_id();
        // let community = self.communities.get(&contract_id).unwrap();
        // assert!(sender_id == community.owner_id || sender_id == self.owner_id, "not owner");
        let code_info = self.codes.get(&community_type).unwrap();
        let hash: Vec<u8> = CryptoHash::from(code_info.hash).to_vec();

        let storage_cost = self.account_storage_usage * env::storage_byte_cost() + u128::from(code_info.storage_deposit);

        assert!(env::attached_deposit() > storage_cost, "not enough deposit");

        let promise = Promise::new(contract_id.clone())
        .function_call("upgrade".to_string(), env::storage_read(&hash).unwrap(), u128::from(code_info.storage_deposit), (env::prepaid_gas() - env::used_gas()) / 4);
        let promise = match migrate {
            true => {
                promise.function_call("migrate".to_string(), json!({
                    "args": json!(self.args)
                }).to_string().as_bytes().to_vec(), 0, (env::prepaid_gas() - env::used_gas()) / 4)
            },
            false => promise
        };
        promise.then(
            Promise::new(env::current_account_id()).function_call("on_update_community".to_string(), json!({
                "contract_id": contract_id,
                "community_type": community_type
            }).to_string().as_bytes().to_vec(), 0, (env::prepaid_gas() - env::used_gas()) / 4)
        );
    }
}



#[cfg(test)]
mod tests {
    use near_sdk::bs58;

    use crate::utils::verify;


    #[test]
    pub fn test_verify() {
        let nonce = "G00MHfp2Km".to_string();
        let sign = bs58::decode("ZdGAt1WdMn1nZ5Z8om2dJiqCCaSopbMAkgtkfFPTLDxyyzy3kkfXr679Af7SgXCNw7zbv9WPTqvNndPcyjJBiDm").into_vec().unwrap();
        let pk = bs58::decode("6wE48MezRaLCu8RkTQ5RxQnKbqwZVqqtyj7DxVJdHfib").into_vec().unwrap();
        assert!(verify(nonce.as_bytes(), &sign, &pk), "invalid code")
    }

}