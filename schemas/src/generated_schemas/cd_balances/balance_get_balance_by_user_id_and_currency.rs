// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceGetBalanceByUserIdAndCurrencyParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
}
impl Schema for CdBalancesBalanceGetBalanceByUserIdAndCurrencyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"currency\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currency\"]}") . unwrap ()
    }
}
impl Agent for CdBalancesBalanceGetBalanceByUserIdAndCurrencyParams {
    fn topic() -> &'static str {
        "cd-balances_balance_getBalanceByUserIdAndCurrency"
    }
    fn method() -> &'static str {
        "balance_getBalanceByUserIdAndCurrency"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceGetBalanceByUserIdAndCurrencyReturns {
    #[serde(rename = "totalEarned")]
    pub total_earned: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for CdBalancesBalanceGetBalanceByUserIdAndCurrencyReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalEarned\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"amount\",\"totalEarned\"]}")
    }
}
impl Agent for CdBalancesBalanceGetBalanceByUserIdAndCurrencyReturns {
    fn topic() -> &'static str {
        "cd-balances_balance_getBalanceByUserIdAndCurrency"
    }
    fn method() -> &'static str {
        "balance_getBalanceByUserIdAndCurrency"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
