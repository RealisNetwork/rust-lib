// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBattlePassGenerateProductParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalTypes")]
    pub personal_types: Vec<String>,
}
impl Schema for OrchestratorBattlePassGenerateProductParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type OrchestratorBattlePassGenerateProductReturns = bool;
