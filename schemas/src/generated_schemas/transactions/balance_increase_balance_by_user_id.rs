// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceIncreaseBalanceByUserIdParamsExtraDetailsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceIncreaseBalanceByUserIdParams {
    #[serde(rename = "extraDetails")]
    pub extra_details: Option<TransactionsBalanceIncreaseBalanceByUserIdParamsExtraDetailsParams>,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "creator")]
    pub creator: String,
    #[serde(rename = "reason")]
    pub reason: String,
}
impl Schema for TransactionsBalanceIncreaseBalanceByUserIdParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceIncreaseBalanceByUserIdReturns {}
