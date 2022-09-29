// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesDeleteParams {
    #[serde(rename = "lockedFunds")]
    pub locked_funds: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundBalancesDeleteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lockedFunds\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"lockedFunds\"]}") . unwrap ()
    }
}
impl Agent for RefundBalancesDeleteParams {
    fn topic() -> &'static str {
        "refund_balances_delete"
    }
    fn method() -> &'static str {
        "balances_delete"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesDeleteReturns(pub bool);
impl Schema for RefundBalancesDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for RefundBalancesDeleteReturns {
    fn topic() -> &'static str {
        "refund_balances_delete"
    }
    fn method() -> &'static str {
        "balances_delete"
    }
    fn agent() -> &'static str {
        "refund"
    }
}
