// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceIncreaseUserBalanceParams {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "topicToSuccessResponse")]
    pub topic_to_success_response: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
}
impl Schema for OrchestratorBalanceIncreaseUserBalanceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)$\"},\"amount\":{\"type\":\"string\"},\"topicToSuccessResponse\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}")
    }
}
impl Agent for OrchestratorBalanceIncreaseUserBalanceParams {
    fn topic() -> &'static str {
        "orchestrator_balance_increaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_increaseUserBalance"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceIncreaseUserBalanceReturns(bool);
impl Schema for OrchestratorBalanceIncreaseUserBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorBalanceIncreaseUserBalanceReturns {
    fn topic() -> &'static str {
        "orchestrator_balance_increaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_increaseUserBalance"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
