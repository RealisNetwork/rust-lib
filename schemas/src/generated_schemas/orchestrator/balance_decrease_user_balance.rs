// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceDecreaseUserBalanceParams {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "topicToSuccessResponse")]
    pub topic_to_success_response: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "currency")]
    pub currency: String,
}
impl Schema for OrchestratorBalanceDecreaseUserBalanceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"topicToSuccessResponse\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}") . unwrap ()
    }
}
impl Agent for OrchestratorBalanceDecreaseUserBalanceParams {
    fn topic() -> &'static str {
        "orchestrator_balance_decreaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_decreaseUserBalance"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceDecreaseUserBalanceReturns(pub bool);
impl Schema for OrchestratorBalanceDecreaseUserBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorBalanceDecreaseUserBalanceReturns {
    fn topic() -> &'static str {
        "orchestrator_balance_decreaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_decreaseUserBalance"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
