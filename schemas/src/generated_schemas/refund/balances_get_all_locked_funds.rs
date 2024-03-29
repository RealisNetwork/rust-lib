// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllLockedFundsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundBalancesGetAllLockedFundsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for RefundBalancesGetAllLockedFundsParams {
    fn topic() -> &'static str {
        "refund_balances_getAllLockedFunds"
    }
    fn method() -> &'static str {
        "balances_getAllLockedFunds"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllLockedFundsReturns(pub bool);
impl Schema for RefundBalancesGetAllLockedFundsReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for RefundBalancesGetAllLockedFundsReturns {
    fn topic() -> &'static str {
        "refund_balances_getAllLockedFunds"
    }
    fn method() -> &'static str {
        "balances_getAllLockedFunds"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
