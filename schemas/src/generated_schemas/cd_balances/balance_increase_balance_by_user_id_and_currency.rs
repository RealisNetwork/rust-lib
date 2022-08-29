// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "currencyType")]
    pub currency_type: String,
}
impl Schema for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"currencyType\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currencyType\",\"amount\",\"txId\"]}")
    }
}
impl Agent for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyParams {
    fn topic() -> &'static str {
        "cd-balances_balance_increaseBalanceByUserIdAndCurrency"
    }
    fn method() -> &'static str {
        "balance_increaseBalanceByUserIdAndCurrency"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}
impl<'de> Deserialize<'de> for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns;
impl Schema for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns {
    fn topic() -> &'static str {
        "cd-balances_balance_increaseBalanceByUserIdAndCurrency"
    }
    fn method() -> &'static str {
        "balance_increaseBalanceByUserIdAndCurrency"
    }
    fn agent() -> &'static str {
        "cd-balances"
    }
}