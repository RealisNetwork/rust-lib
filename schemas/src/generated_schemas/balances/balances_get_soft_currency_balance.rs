// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BalancesBalancesGetSoftCurrencyBalanceParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(BalancesBalancesGetSoftCurrencyBalanceParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BalancesBalancesGetSoftCurrencyBalanceParams;
impl Schema for BalancesBalancesGetSoftCurrencyBalanceParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for BalancesBalancesGetSoftCurrencyBalanceParams {
    fn topic() -> &'static str {
        "balances_balances_getSoftCurrencyBalance"
    }
    fn method() -> &'static str {
        "balances_getSoftCurrencyBalance"
    }
    fn agent() -> &'static str {
        "balances"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesGetSoftCurrencyBalanceReturns {
    #[serde(rename = "balance")]
    pub balance: i32,
}
impl Schema for BalancesBalancesGetSoftCurrencyBalanceReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"balance\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"balance\"]}")
    }
}
impl Agent for BalancesBalancesGetSoftCurrencyBalanceReturns {
    fn topic() -> &'static str {
        "balances_balances_getSoftCurrencyBalance"
    }
    fn method() -> &'static str {
        "balances_getSoftCurrencyBalance"
    }
    fn agent() -> &'static str {
        "balances"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
