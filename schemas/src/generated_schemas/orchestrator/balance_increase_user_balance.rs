// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceIncreaseUserBalanceParams {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "topicToSuccessResponse")]
    pub topic_to_success_response: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for OrchestratorBalanceIncreaseUserBalanceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)$\"},\"txId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"topicToSuccessResponse\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceIncreaseUserBalanceReturns(bool);
impl Schema for OrchestratorBalanceIncreaseUserBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
