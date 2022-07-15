// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsBalanceUpdateTransactionHashAndBlockIdParams {
    #[serde(rename = "newTxId")]
    pub new_tx_id: String,
    #[serde(rename = "oldTxId")]
    pub old_tx_id: String,
    #[serde(rename = "newBlockId")]
    pub new_block_id: String,
}
pub type TransactionsBalanceUpdateTransactionHashAndBlockIdReturns = bool;
