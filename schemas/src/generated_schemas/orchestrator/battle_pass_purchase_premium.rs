// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBattlePassPurchasePremiumParams {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for OrchestratorBattlePassPurchasePremiumParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"txId\",\"creator\"]}")
    }
}
impl<'de> Deserialize<'de> for OrchestratorBattlePassPurchasePremiumReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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