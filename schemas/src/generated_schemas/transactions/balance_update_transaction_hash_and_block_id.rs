// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceUpdateTransactionHashAndBlockIdParams {
    #[serde(rename = "newBlockId")]
    pub new_block_id: String,
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
}
impl Schema for TransactionsBalanceUpdateTransactionHashAndBlockIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"newBlockId\":{\"type\":\"string\"},\"oldTxId\":{\"type\":\"string\"},\"newTxId\":{\"type\":\"string\"}},\"required\":[\"oldTxId\",\"newTxId\",\"newBlockId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceUpdateTransactionHashAndBlockIdParams {
    fn topic() -> &'static str {
        "transactions_balance_updateTransactionHashAndBlockId"
    }
    fn method() -> &'static str {
        "balance_updateTransactionHashAndBlockId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceUpdateTransactionHashAndBlockIdReturns(pub bool);
impl Schema for TransactionsBalanceUpdateTransactionHashAndBlockIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsBalanceUpdateTransactionHashAndBlockIdReturns {
    fn topic() -> &'static str {
        "transactions_balance_updateTransactionHashAndBlockId"
    }
    fn method() -> &'static str {
        "balance_updateTransactionHashAndBlockId"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
