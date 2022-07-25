// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBattlePassPurchaseExperienceParams {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for OrchestratorBattlePassPurchaseExperienceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"creator\":{\"type\":\"string\"},\"amount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"txId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"txId\",\"creator\",\"amount\"]}")
    }
}
impl Agent for OrchestratorBattlePassPurchaseExperienceParams {
    fn topic() -> &'static str {
        "orchestrator_battlePass_purchaseExperience"
    }
    fn method() -> &'static str {
        "battlePass_purchaseExperience"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
impl<'de> Deserialize<'de> for OrchestratorBattlePassPurchaseExperienceReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OrchestratorBattlePassPurchaseExperienceReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct OrchestratorBattlePassPurchaseExperienceReturns;
impl Schema for OrchestratorBattlePassPurchaseExperienceReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for OrchestratorBattlePassPurchaseExperienceReturns {
    fn topic() -> &'static str {
        "orchestrator_battlePass_purchaseExperience"
    }
    fn method() -> &'static str {
        "battlePass_purchaseExperience"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
