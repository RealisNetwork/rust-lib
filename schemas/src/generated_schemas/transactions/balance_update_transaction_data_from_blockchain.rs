// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceUpdateTransactionDataFromBlockchainParams {
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
}
impl Schema for TransactionsBalanceUpdateTransactionDataFromBlockchainParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"newTxId\":{\"type\":\"string\"},\"oldTxId\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"}},\"required\":[\"oldTxId\",\"newTxId\",\"blockId\"]}") . unwrap ()
    }
}
impl Agent for TransactionsBalanceUpdateTransactionDataFromBlockchainParams {
    fn topic() -> &'static str {
        "transactions_balance_updateTransactionDataFromBlockchain"
    }
    fn method() -> &'static str {
        "balance_updateTransactionDataFromBlockchain"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceUpdateTransactionDataFromBlockchainReturns(pub bool);
impl Schema for TransactionsBalanceUpdateTransactionDataFromBlockchainReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for TransactionsBalanceUpdateTransactionDataFromBlockchainReturns {
    fn topic() -> &'static str {
        "transactions_balance_updateTransactionDataFromBlockchain"
    }
    fn method() -> &'static str {
        "balance_updateTransactionDataFromBlockchain"
    }
    fn agent() -> &'static str {
        "transactions"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
