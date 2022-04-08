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

    pub fn add_code_type(&mut self, token_type: String, length: u32, hash: String) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        self.codes.insert(&token_type, &CodeInfo {
            length: length,
            hash: hash,
            storage_deposit: U128::from((length + 20000) as u128 * env::storage_byte_cost())
        });
    }

    pub fn del_code_type(&mut self, token_type: String) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        assert!(self.codes.get(&token_type).is_some(), "not exist");
        self.codes.remove(&token_type);
        env::storage_remove(token_type.as_bytes());
    }

    pub fn add_code(&mut self, token_type: String, code: Vec<u8>) {
        assert!(self.owner_id == env::predecessor_account_id(), "contract owner only");
        let old_code = env::storage_read(token_type.as_bytes()).unwrap_or(Vec::new());
        env::storage_write(token_type.as_bytes(), &[old_code, code].concat());
    }
}
