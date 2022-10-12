// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceDecreaseUserBalanceParams {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "topicToSuccessResponse")]
    pub topic_to_success_response: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for PurchaseBalanceDecreaseUserBalanceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"creator\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"topicToSuccessResponse\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}") . unwrap ()
    }
}
impl Agent for PurchaseBalanceDecreaseUserBalanceParams {
    fn topic() -> &'static str {
        "purchase_balance_decreaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_decreaseUserBalance"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceDecreaseUserBalanceReturns(pub bool);
impl Schema for PurchaseBalanceDecreaseUserBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for PurchaseBalanceDecreaseUserBalanceReturns {
    fn topic() -> &'static str {
        "purchase_balance_decreaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_decreaseUserBalance"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
