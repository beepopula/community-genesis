use near_sdk::PublicKey;

use crate::*;

#[near_bindgen]
impl CommunityGenesis {
    pub fn get_args(&self) -> HashMap<String, String> {
        self.args.clone()
    }

    pub fn set_args(&mut self, args: HashMap<String, String>) {
        let sender = env::predecessor_account_id();
        assert!(sender == self.owner_id, "owner only");
        self.args = args;
    }

    pub fn add_code_type(&mut self, community_type: String, length: u32, hash: Base58CryptoHash) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        if let Some(code) = self.codes.get(&community_type) {
            env::storage_remove(&CryptoHash::from(code.hash).to_vec());
        }
        self.codes.insert(&community_type, &CodeInfo {
            length: length,
            hash: hash,
            storage_deposit: U128::from((length + 20000) as u128 * env::storage_byte_cost())
        });
    }

    pub fn del_code_type(&mut self, community_type: String) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        let code = self.codes.get(&community_type).unwrap();
        env::storage_remove(&CryptoHash::from(code.hash).to_vec());
        self.codes.remove(&community_type);
        
    }

    pub fn del_code_hash(&mut self, hash: Base58CryptoHash) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        let hash = CryptoHash::from(hash).to_vec();
        env::storage_remove(&hash);
    }

    pub fn del_community(&mut self, contract_id: AccountId) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        Promise::new(contract_id)
        .function_call("del_contract".to_string(), json!({}).to_string().as_bytes().to_vec(), 1, (env::prepaid_gas() - env::used_gas()) / 2);
    }

    #[payable]
    pub fn deploy_community_by_owner(&mut self, name: String, community_type: String, creator_id: AccountId) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
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
            "owner_id": creator_id,
            "args": json!(self.args) 
        }).to_string().as_bytes().to_vec(), 0, (env::prepaid_gas() - env::used_gas()) / 3).then(
            Promise::new(env::current_account_id())
            .function_call("on_add_community".into(), json!({
                "contract_id": contract_id,
                "community_type": community_type,
                "owner_id": creator_id,
            }).to_string().into(), env::attached_deposit(), (env::prepaid_gas() - env::used_gas()) / 3)
        );
    }
}

#[no_mangle]
pub extern "C" fn add_code() {
    env::setup_panic_hook();
    let contract: CommunityGenesis = env::state_read().unwrap();
    assert!(contract.owner_id == env::predecessor_account_id(), "contract owner only");
    let input = env::input().unwrap();
    let hash = env::sha256(&input);
    env::storage_write(&hash, &input);
    env::value_return(&hash);
}
