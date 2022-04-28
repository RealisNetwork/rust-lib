use runtime::AccountId;
use serde::{Deserialize, Serialize};
use substrate_api_client::Hash;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TempSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    pub id: String,
    pub params: TempParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TempParams {
    pub hash: Hash,
}

// TODO: restruct to rust-lib standard 
#[derive(Deserialize, Serialize, Clone, Debug)]
pub(super) struct AuthInfo {
    pub account_id: AccountId,
}