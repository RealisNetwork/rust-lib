// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
        todo!()
    }
}
impl<'de> Deserialize<'de> for OrchestratorBattlePassPurchasePremiumReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OrchestratorBattlePassPurchasePremiumReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct OrchestratorBattlePassPurchasePremiumReturns;
