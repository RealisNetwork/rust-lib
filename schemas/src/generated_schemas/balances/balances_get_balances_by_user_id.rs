// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for BalancesBalancesGetBalancesByUserIdParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BalancesBalancesGetBalancesByUserIdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BalancesBalancesGetBalancesByUserIdParams;
impl Schema for BalancesBalancesGetBalancesByUserIdParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesGetBalancesByUserIdReturns {
    #[serde(rename = "freeExperience")]
    pub free_experience: i32,
    #[serde(rename = "softCurrency")]
    pub soft_currency: i32,
}
impl Schema for BalancesBalancesGetBalancesByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"freeExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"softCurrency\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"softCurrency\",\"freeExperience\"]}")
    }
}
