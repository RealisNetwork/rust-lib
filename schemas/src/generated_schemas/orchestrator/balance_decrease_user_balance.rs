// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBalanceDecreaseUserBalanceParams {
    #[serde(
        rename = "topicToSuccessResponse",
        deserialize_with = "deserialize_to_string"
    )]
    pub topic_to_success_response: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "currency", deserialize_with = "deserialize_to_string")]
    pub currency: String,
    #[serde(rename = "amount", deserialize_with = "deserialize_to_string")]
    pub amount: String,
    #[serde(rename = "txId", deserialize_with = "deserialize_to_string")]
    pub tx_id: String,
    #[serde(rename = "creator", deserialize_with = "deserialize_to_string")]
    pub creator: String,
}
impl Schema for OrchestratorBalanceDecreaseUserBalanceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"topicToSuccessResponse\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"amount\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
