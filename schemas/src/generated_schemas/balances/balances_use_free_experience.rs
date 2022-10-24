// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesUseFreeExperienceParams {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "amount")]
    pub amount: i32,
}
impl Schema for BalancesBalancesUseFreeExperienceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"amount\",\"txId\"]}") . unwrap ()
    }
}
impl Agent for BalancesBalancesUseFreeExperienceParams {
    fn topic() -> &'static str {
        "balances_balances_useFreeExperience"
    }
    fn method() -> &'static str {
        "balances_useFreeExperience"
    }
    fn agent() -> &'static str {
        "balances"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesUseFreeExperienceReturns(pub bool);
impl Schema for BalancesBalancesUseFreeExperienceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for BalancesBalancesUseFreeExperienceReturns {
    fn topic() -> &'static str {
        "balances_balances_useFreeExperience"
    }
    fn method() -> &'static str {
        "balances_useFreeExperience"
    }
    fn agent() -> &'static str {
        "balances"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
