use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByBinanceSchema {
    pub id: String,
    #[serde(rename = "topicRes", alias = "topicResponse")]
    pub topic_res: String,
    pub params: GetUserByBinanceSchemaParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByBinanceSchemaParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
}
