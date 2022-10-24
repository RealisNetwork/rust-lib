// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesAddParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "lockedFunds", deserialize_with = "deserialize_to_string")]
    pub locked_funds: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundBalancesAddParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lockedFunds\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"lockedFunds\"]}") . unwrap ()
    }
}
impl Agent for RefundBalancesAddParams {
    fn topic() -> &'static str {
        "refund_balances_add"
    }
    fn method() -> &'static str {
        "balances_add"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesAddReturns(pub bool);
impl Schema for RefundBalancesAddReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for RefundBalancesAddReturns {
    fn topic() -> &'static str {
        "refund_balances_add"
    }
    fn method() -> &'static str {
        "balances_add"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
