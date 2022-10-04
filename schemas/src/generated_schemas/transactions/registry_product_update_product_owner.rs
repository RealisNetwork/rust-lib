// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductUpdateProductOwnerParams {
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
}
impl Schema for TransactionsRegistryProductUpdateProductOwnerParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"string\"},\"ownerId\":{\"type\":\"string\"}},\"required\":[\"productId\",\"ownerId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsRegistryProductUpdateProductOwnerParams {
    fn topic() -> &'static str {
        "transactions_registryProduct_updateProductOwner"
    }
    fn method() -> &'static str {
        "registryProduct_updateProductOwner"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductUpdateProductOwnerReturns(pub bool);
impl Schema for TransactionsRegistryProductUpdateProductOwnerReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsRegistryProductUpdateProductOwnerReturns {
    fn topic() -> &'static str {
        "transactions_registryProduct_updateProductOwner"
    }
    fn method() -> &'static str {
        "registryProduct_updateProductOwner"
    }
    fn agent() -> &'static str {
        "transactions"
    }
}
