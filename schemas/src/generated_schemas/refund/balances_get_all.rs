// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for RefundBalancesGetAllParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllReturnsParams {
    #[serde(rename = "lockedFunds")]
    pub locked_funds: String,
    #[serde(rename = "currency")]
    pub currency: (),
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "lockedUntil")]
    pub locked_until: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundBalancesGetAllReturns(pub Vec<RefundBalancesGetAllReturnsParams>);
impl Schema for RefundBalancesGetAllReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"lockedFunds\":{\"type\":\"string\"},\"currency\":{},\"createdAt\":{\"type\":\"string\",\"format\":\"date\"},\"userId\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\",\"format\":\"date\"},\"lockedUntil\":{\"type\":\"string\",\"format\":\"date\"}},\"required\":[\"userId\",\"lockedFunds\",\"currency\",\"lockedUntil\",\"createdAt\",\"updatedAt\"]}}")
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
}
