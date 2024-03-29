// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorProductBurnParams {
    #[serde(rename = "productIds")]
    pub product_ids: Vec<f64>,
}
impl Schema for OrchestratorProductBurnParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productIds\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}},\"required\":[\"productIds\"]}") . unwrap ()
    }
}
impl Agent for OrchestratorProductBurnParams {
    fn topic() -> &'static str {
        "orchestrator_product_burn"
    }
    fn method() -> &'static str {
        "product_burn"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorProductBurnReturns(pub bool);
impl Schema for OrchestratorProductBurnReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorProductBurnReturns {
    fn topic() -> &'static str {
        "orchestrator_product_burn"
    }
    fn method() -> &'static str {
        "product_burn"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
