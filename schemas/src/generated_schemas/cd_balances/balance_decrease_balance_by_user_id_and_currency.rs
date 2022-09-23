// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyParams {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "currencyType")]
    pub currency_type: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"currencyType\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currencyType\",\"amount\",\"txId\"]}")
    }
}
impl Agent for CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyParams {
    fn topic() -> &'static str {
        "cd-balances_balance_decreaseBalanceByUserIdAndCurrency"
    }
    fn method() -> &'static str {
        "balance_decreaseBalanceByUserIdAndCurrency"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
impl<'de> Deserialize<'de> for CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyReturns;
impl Schema for CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyReturns {
    fn topic() -> &'static str {
        "cd-balances_balance_decreaseBalanceByUserIdAndCurrency"
    }
    fn method() -> &'static str {
        "balance_decreaseBalanceByUserIdAndCurrency"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
