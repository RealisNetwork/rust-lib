// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for PurchaseBalanceUserBalanceChangedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(PurchaseBalanceUserBalanceChangedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct PurchaseBalanceUserBalanceChangedNotificationParams;
impl Schema for PurchaseBalanceUserBalanceChangedNotificationParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseBalanceUserBalanceChangedNotificationReturns {
    #[serde(rename = "currency", deserialize_with = "deserialize_to_string")]
    pub currency: String,
    #[serde(rename = "transactionHash", deserialize_with = "deserialize_to_string")]
    pub transaction_hash: String,
    #[serde(rename = "amount", deserialize_with = "deserialize_to_string")]
    pub amount: String,
    #[serde(rename = "blockId", deserialize_with = "deserialize_to_string")]
    pub block_id: String,
    #[serde(rename = "balance", deserialize_with = "deserialize_to_string")]
    pub balance: String,
}
impl Schema for PurchaseBalanceUserBalanceChangedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"transactionHash\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"},\"balance\":{\"type\":\"string\"}},\"required\":[\"currency\",\"amount\",\"transactionHash\",\"balance\",\"blockId\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
