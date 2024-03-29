// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductDeleteProductByUserIdParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for TransactionsRegistryProductDeleteProductByUserIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsRegistryProductDeleteProductByUserIdParams {
    fn topic() -> &'static str {
        "transactions_registryProduct_deleteProductByUserId"
    }
    fn method() -> &'static str {
        "registryProduct_deleteProductByUserId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRegistryProductDeleteProductByUserIdReturns(pub bool);
impl Schema for TransactionsRegistryProductDeleteProductByUserIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsRegistryProductDeleteProductByUserIdReturns {
    fn topic() -> &'static str {
        "transactions_registryProduct_deleteProductByUserId"
    }
    fn method() -> &'static str {
        "registryProduct_deleteProductByUserId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
