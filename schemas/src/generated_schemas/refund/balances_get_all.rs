// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for RefundBalancesGetAllParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for RefundBalancesGetAllParams {
    fn topic() -> &'static str {
        "refund_balances_getAll"
    }
    fn method() -> &'static str {
        "balances_getAll"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllReturnsParams {
    #[serde(rename = "lockedUntil", deserialize_with = "deserialize_to_string")]
    pub locked_until: String,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_to_string")]
    pub created_at: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "lockedFunds", deserialize_with = "deserialize_to_string")]
    pub locked_funds: String,
    #[serde(rename = "updatedAt", deserialize_with = "deserialize_to_string")]
    pub updated_at: String,
    #[serde(rename = "currency")]
    pub currency: (),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllReturns(pub Vec<RefundBalancesGetAllReturnsParams>);
impl Schema for RefundBalancesGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"lockedUntil\":{\"type\":\"string\",\"format\":\"date\"},\"createdAt\":{\"type\":\"string\",\"format\":\"date\"},\"userId\":{\"type\":\"string\"},\"lockedFunds\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\",\"format\":\"date\"},\"currency\":{}},\"required\":[\"userId\",\"lockedFunds\",\"currency\",\"lockedUntil\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for RefundBalancesGetAllReturns {
    fn topic() -> &'static str {
        "refund_balances_getAll"
    }
    fn method() -> &'static str {
        "balances_getAll"
    }
    fn agent() -> &'static str {
        "refund"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
