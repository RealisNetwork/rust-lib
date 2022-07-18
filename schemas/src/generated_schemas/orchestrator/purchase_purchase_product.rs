// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorPurchasePurchaseProductParams {
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "productType")]
    pub product_type: String,
}
impl Schema for OrchestratorPurchasePurchaseProductParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type OrchestratorPurchasePurchaseProductReturns = bool;
