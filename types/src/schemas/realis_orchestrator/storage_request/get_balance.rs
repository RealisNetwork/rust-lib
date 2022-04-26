use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use runtime::AccountId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalanceSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}
