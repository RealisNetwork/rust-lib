// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceUpdateTransactionDataFromBlockchainParams {
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
}
impl Schema for TransactionsBalanceUpdateTransactionDataFromBlockchainParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type TransactionsBalanceUpdateTransactionDataFromBlockchainReturns = bool;
