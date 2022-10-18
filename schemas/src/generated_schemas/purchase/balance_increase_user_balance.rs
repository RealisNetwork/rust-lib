// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceIncreaseUserBalanceParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "topicToSuccessResponse")]
    pub topic_to_success_response: String,
}
impl Schema for PurchaseBalanceIncreaseUserBalanceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"topicToSuccessResponse\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\",\"amount\",\"creator\",\"txId\",\"topicToSuccessResponse\"]}") . unwrap ()
    }
}
impl Agent for PurchaseBalanceIncreaseUserBalanceParams {
    fn topic() -> &'static str {
        "purchase_balance_increaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_increaseUserBalance"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceIncreaseUserBalanceReturns(pub bool);
impl Schema for PurchaseBalanceIncreaseUserBalanceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for PurchaseBalanceIncreaseUserBalanceReturns {
    fn topic() -> &'static str {
        "purchase_balance_increaseUserBalance"
    }
    fn method() -> &'static str {
        "balance_increaseUserBalance"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
