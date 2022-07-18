// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for PurchaseBalanceUserBalanceChangedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(PurchaseBalanceUserBalanceChangedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct PurchaseBalanceUserBalanceChangedNotificationParams;
impl Schema for PurchaseBalanceUserBalanceChangedNotificationParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceUserBalanceChangedNotificationReturns {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
}
