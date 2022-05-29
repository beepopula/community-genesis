use crate::*;

#[near_bindgen]
impl CommunityGenesis {
    pub fn get_public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn set_public_key(&mut self, public_key: String) {
        let sender = env::predecessor_account_id();
        assert!(sender == self.owner_id, "owner only");
        self.public_key = public_key;
    }

    pub fn add_code_type(&mut self, community_type: String, length: u32, hash: Base58CryptoHash) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
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
