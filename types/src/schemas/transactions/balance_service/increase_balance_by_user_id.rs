use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IncreaseBalanceByUserIdSchema {
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
