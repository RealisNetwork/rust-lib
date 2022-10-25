// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyParams {
    #[serde(rename = "currencyType", deserialize_with = "deserialize_to_string")]
    pub currency_type: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "txId", deserialize_with = "deserialize_to_string")]
    pub tx_id: String,
    #[serde(rename = "amount", deserialize_with = "deserialize_to_string")]
    pub amount: String,
}
impl Schema for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"currencyType\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"userId\",\"currencyType\",\"amount\",\"txId\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
impl<'de> Deserialize<'de> for CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
