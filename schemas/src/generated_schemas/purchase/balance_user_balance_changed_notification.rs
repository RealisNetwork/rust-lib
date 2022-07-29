// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for PurchaseBalanceUserBalanceChangedNotificationParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
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
        serde_json::json!("{}")
    }
}
impl Agent for PurchaseBalanceUserBalanceChangedNotificationParams {
    fn topic() -> &'static str {
        "purchase_balance_userBalanceChangedNotification"
    }
    fn method() -> &'static str {
        "balance_userBalanceChangedNotification"
    }
    fn agent() -> &'static str {
        "purchase"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceUserBalanceChangedNotificationReturns {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "balance")]
    pub balance: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
}
impl Schema for PurchaseBalanceUserBalanceChangedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)$\"},\"amount\":{\"type\":\"string\"},\"balance\":{\"type\":\"string\"},\"transactionHash\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"}},\"required\":[\"currency\",\"amount\",\"transactionHash\",\"balance\",\"blockId\"]}")
    }
}
impl Agent for PurchaseBalanceUserBalanceChangedNotificationReturns {
    fn topic() -> &'static str {
        "purchase_balance_userBalanceChangedNotification"
    }
    fn method() -> &'static str {
        "balance_userBalanceChangedNotification"
    }
    fn agent() -> &'static str {
        "purchase"
    }
}
