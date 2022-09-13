// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBattlePassGenerateProductParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalTypes")]
    pub personal_types: Vec<String>,
}
impl Schema for OrchestratorBattlePassGenerateProductParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"personalTypes\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"userId\",\"personalTypes\"]}")
    }
}
impl Agent for OrchestratorBattlePassGenerateProductParams {
    fn topic() -> &'static str {
        "orchestrator_battlePass_generateProduct"
    }
    fn method() -> &'static str {
        "battlePass_generateProduct"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBattlePassGenerateProductReturns(pub bool);
impl Schema for OrchestratorBattlePassGenerateProductReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorBattlePassGenerateProductReturns {
    fn topic() -> &'static str {
        "orchestrator_battlePass_generateProduct"
    }
    fn method() -> &'static str {
        "battlePass_generateProduct"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
