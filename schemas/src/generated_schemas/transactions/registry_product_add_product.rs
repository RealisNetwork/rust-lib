// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductAddProductParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for TransactionsRegistryProductAddProductParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"productId\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"}},\"required\":[\"userId\",\"productId\",\"personalType\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductAddProductReturns(bool);
impl Schema for TransactionsRegistryProductAddProductReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}