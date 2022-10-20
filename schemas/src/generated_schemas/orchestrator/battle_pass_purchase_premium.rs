// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBattlePassPurchasePremiumParams {
    #[serde(rename = "txId", deserialize_with = "deserialize_to_string")]
    pub tx_id: String,
    #[serde(rename = "creator", deserialize_with = "deserialize_to_string")]
    pub creator: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for OrchestratorBattlePassPurchasePremiumParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"txId\",\"creator\"]}") . unwrap ()
    }
}
impl Agent for OrchestratorBattlePassPurchasePremiumParams {
    fn topic() -> &'static str {
        "orchestrator_battlePass_purchasePremium"
    }
    fn method() -> &'static str {
        "battlePass_purchasePremium"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
impl<'de> Deserialize<'de> for OrchestratorBattlePassPurchasePremiumReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(OrchestratorBattlePassPurchasePremiumReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct OrchestratorBattlePassPurchasePremiumReturns;
impl Schema for OrchestratorBattlePassPurchasePremiumReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for OrchestratorBattlePassPurchasePremiumReturns {
    fn topic() -> &'static str {
        "orchestrator_battlePass_purchasePremium"
    }
    fn method() -> &'static str {
        "battlePass_purchasePremium"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
