use crate::requests::AuthInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IncreaseBalanceByUserIdSchema {
    pub id: String,
    #[serde(rename = "topicResponse", alias = "topicRes")]
    pub topic_res: String,
    pub params: IncreaseBalanceByUserIdSchemaParams,
    #[serde(rename = "authInfo")]
    pub auth_info: AuthInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IncreaseBalanceByUserIdSchemaParams {
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<Value>,
    pub creator: String,
    pub reason: String,
    pub currency: String,
    pub amount: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
