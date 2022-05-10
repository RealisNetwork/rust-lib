use runtime::AccountId;
use serde::{Deserialize, Serialize};

// Schema for serialize/deserialize json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserNftListSchema {
    #[serde(rename = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
    pub id: String,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}
