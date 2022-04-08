use crate::*;

#[near_bindgen]
impl CommunityGenesis {
    

    pub fn get_account_communities(&self, account_id: AccountId) -> Vec<AccountId> {
        let empty_vec: Vec<AccountId> = Vec::new();
        let tokens = self.accounts.get(&account_id).unwrap_or(empty_vec);
        tokens.clone()
    }

    pub fn get_communities(&self) -> Vec<AccountId> {
        self.communities.keys().collect()
    }

    pub fn get_community_info(&self, contract_id: AccountId) -> Community {
        self.communities.get(&contract_id).unwrap()
    }

    pub fn get_code_info(&self, community_type: String) -> CodeInfo {
        self.codes.get(&community_type).unwrap()
    }

    pub fn get_code_storage_cost(&self, community_type: String) -> U128{
        let code_info = self.codes.get(&community_type).unwrap();
        let storage_cost = self.account_storage_usage * env::storage_byte_cost() + u128::from(code_info.storage_deposit);
        storage_cost.into()
    }

    pub fn get_code_types(&self) -> Vec<String> {
        self.codes.keys().collect()
    }
}