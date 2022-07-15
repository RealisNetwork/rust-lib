// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsBalanceGetNumWithFilterParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "credit")]
    pub credit: String,
    #[serde(rename = "debit")]
    pub debit: String,
}
pub type TransactionsBalanceGetNumWithFilterReturns = i64;
