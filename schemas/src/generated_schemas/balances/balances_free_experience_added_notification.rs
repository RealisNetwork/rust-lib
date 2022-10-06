// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BalancesBalancesFreeExperienceAddedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(BalancesBalancesFreeExperienceAddedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BalancesBalancesFreeExperienceAddedNotificationParams;
impl Schema for BalancesBalancesFreeExperienceAddedNotificationParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for BalancesBalancesFreeExperienceAddedNotificationParams {
    fn topic() -> &'static str {
        "balances_balances_freeExperienceAddedNotification"
    }
    fn method() -> &'static str {
        "balances_freeExperienceAddedNotification"
    }
    fn agent() -> &'static str {
        "balances"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesFreeExperienceAddedNotificationReturns {
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "balance")]
    pub balance: i32,
}
impl Schema for BalancesBalancesFreeExperienceAddedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"balance\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"amount\",\"balance\"]}")
    }
}
impl Agent for BalancesBalancesFreeExperienceAddedNotificationReturns {
    fn topic() -> &'static str {
        "balances_balances_freeExperienceAddedNotification"
    }
    fn method() -> &'static str {
        "balances_freeExperienceAddedNotification"
    }
    fn agent() -> &'static str {
        "balances"
    }
}
