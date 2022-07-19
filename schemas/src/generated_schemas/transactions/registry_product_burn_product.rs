// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductBurnProductParams {
    #[serde(rename = "productId")]
    pub product_id: String,
}
impl Schema for TransactionsRegistryProductBurnProductParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"string\"}},\"required\":[\"productId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductBurnProductReturns {
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TransactionsRegistryProductBurnProductReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"personalType\"]}")
    }
}
