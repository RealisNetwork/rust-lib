// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BalancesBalancesGetFreeExperienceBalanceParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(BalancesBalancesGetFreeExperienceBalanceParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BalancesBalancesGetFreeExperienceBalanceParams;
impl Schema for BalancesBalancesGetFreeExperienceBalanceParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for BalancesBalancesGetFreeExperienceBalanceParams {
    fn topic() -> &'static str {
        "balances_balances_getFreeExperienceBalance"
    }
    fn method() -> &'static str {
        "balances_getFreeExperienceBalance"
    }
    fn agent() -> &'static str {
        "balances"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesGetFreeExperienceBalanceReturns {
    #[serde(rename = "balance")]
    pub balance: i32,
}
impl Schema for BalancesBalancesGetFreeExperienceBalanceReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"balance\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"balance\"]}")
    }
}
impl Agent for BalancesBalancesGetFreeExperienceBalanceReturns {
    fn topic() -> &'static str {
        "balances_balances_getFreeExperienceBalance"
    }
    fn method() -> &'static str {
        "balances_getFreeExperienceBalance"
    }
    fn agent() -> &'static str {
        "balances"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
