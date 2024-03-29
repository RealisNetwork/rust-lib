// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBingoRewardParams {
    #[serde(rename = "productIds")]
    pub product_ids: Vec<f64>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for OrchestratorBingoRewardParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productIds\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"userId\",\"productIds\",\"amount\"]}") . unwrap ()
    }
}
impl Agent for OrchestratorBingoRewardParams {
    fn topic() -> &'static str {
        "orchestrator_bingo_reward"
    }
    fn method() -> &'static str {
        "bingo_reward"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBingoRewardReturns(pub bool);
impl Schema for OrchestratorBingoRewardReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorBingoRewardReturns {
    fn topic() -> &'static str {
        "orchestrator_bingo_reward"
    }
    fn method() -> &'static str {
        "bingo_reward"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
