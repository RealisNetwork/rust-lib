// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CdBalancesBalanceGetBalancesByUserIdParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CdBalancesBalanceGetBalancesByUserIdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CdBalancesBalanceGetBalancesByUserIdParams;
impl Schema for CdBalancesBalanceGetBalancesByUserIdParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for CdBalancesBalanceGetBalancesByUserIdParams {
    fn topic() -> &'static str {
        "cd-balances_balance_getBalancesByUserId"
    }
    fn method() -> &'static str {
        "balance_getBalancesByUserId"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceGetBalancesByUserIdReturnsParams {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "totalEarned")]
    pub total_earned: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceGetBalancesByUserIdReturns(
    pub Vec<CdBalancesBalanceGetBalancesByUserIdReturnsParams>,
);
impl Schema for CdBalancesBalanceGetBalancesByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"totalEarned\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"amount\",\"currency\",\"totalEarned\"]}}")
    }
}
impl Agent for CdBalancesBalanceGetBalancesByUserIdReturns {
    fn topic() -> &'static str {
        "cd-balances_balance_getBalancesByUserId"
    }
    fn method() -> &'static str {
        "balance_getBalancesByUserId"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
