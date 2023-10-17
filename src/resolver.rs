use std::io::Read;

use near_sdk::PromiseResult;

use crate::*;
use crate::utils::refund_extra_storage_deposit;

#[near_bindgen]
impl CommunityGenesis {
    #[private]
    #[payable]
    pub fn on_add_community(&mut self, contract_id: AccountId, community_type: String, owner_id: AccountId, options: Option<HashMap<String, String>>) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                
                let code_info = self.codes.get(&community_type).unwrap();
                self.communities.insert(&contract_id.clone(), &Community { 
                    contract_id: contract_id.clone(), 
                    owner_id: owner_id.clone(), 
                    community_type: community_type.clone(),
                    code_hash: code_info.hash
                });

                if get_env() == "near" {
                    let options = options.clone().expect("not allowed");
                    let nonce = options.get("nonce").unwrap();
                    env::storage_write(nonce.as_bytes(), "1".as_bytes());
                }
                
            },
            _ => unimplemented!()
        }

        
    }

    #[private]
    #[payable]
    pub fn on_update_community(&mut self, contract_id: AccountId, community_type: String) {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let code_info = self.codes.get(&community_type).unwrap();
                let community = self.communities.get(&contract_id.clone()).unwrap();
                self.communities.insert(&contract_id.clone(), &Community { 
                    contract_id: contract_id.clone(), 
                    owner_id: community.owner_id.clone(), 
                    community_type: community_type.clone(),
                    code_hash: code_info.hash
                });
            },
            _ => unimplemented!()
        }
    }
}