use std::io::Read;

use near_sdk::PromiseResult;

use crate::*;
use crate::utils::refund_extra_storage_deposit;

#[near_bindgen]
impl CommunityGenesis {
    #[private]
    #[payable]
    pub fn on_add_community(&mut self, contract_id: AccountId, community_type: String, owner_id: AccountId) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let initial_storage_usage = env::storage_usage();
                self.communities.insert(&contract_id.clone(), &Community { 
                    contract_id: contract_id.clone(), 
                    owner_id: owner_id.clone(), 
                    community_type: community_type.clone()
                });
                let mut owner_communities = self.accounts.get(&owner_id).unwrap_or(Vec::new());
                owner_communities.push(contract_id);
                self.accounts.insert(&owner_id, &owner_communities);
                let code_info = self.codes.get(&community_type).unwrap();
                refund_extra_storage_deposit(env::storage_usage() - initial_storage_usage, u128::from(code_info.storage_deposit));
            },
            _ => unimplemented!()
        }

        
    }
}