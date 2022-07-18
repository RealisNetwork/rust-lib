// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct OrchestratorBattlePassGenerateProductParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalTypes")]
    pub personal_types: Vec<String>,
}
pub type OrchestratorBattlePassGenerateProductReturns = bool;
