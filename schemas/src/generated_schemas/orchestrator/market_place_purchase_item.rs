// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorMarketPlacePurchaseItemParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
impl Schema for OrchestratorMarketPlacePurchaseItemParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"creator\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\",\"userId\",\"txId\",\"creator\"]}")
    }
}
impl Agent for OrchestratorMarketPlacePurchaseItemParams {
    fn topic() -> &'static str {
        "orchestrator_marketPlace_purchaseItem"
    }
    fn method() -> &'static str {
        "marketPlace_purchaseItem"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
impl<'de> Deserialize<'de> for OrchestratorMarketPlacePurchaseItemReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(OrchestratorMarketPlacePurchaseItemReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct OrchestratorMarketPlacePurchaseItemReturns;
impl Schema for OrchestratorMarketPlacePurchaseItemReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for OrchestratorMarketPlacePurchaseItemReturns {
    fn topic() -> &'static str {
        "orchestrator_marketPlace_purchaseItem"
    }
    fn method() -> &'static str {
        "marketPlace_purchaseItem"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
}
