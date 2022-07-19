// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBingoRewardParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "productIds")]
    pub product_ids: Vec<i64>,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for OrchestratorBingoRewardParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"productIds\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"amount\":{\"type\":\"string\"}},\"required\":[\"userId\",\"productIds\",\"amount\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorBingoRewardReturns(bool);
impl Schema for OrchestratorBingoRewardReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
