use runtime::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByRealisSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: GetUserByRealisSchemaParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByRealisSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: AccountId,
}
