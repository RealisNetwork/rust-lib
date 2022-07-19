// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletCreateWalletParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for BinanceWalletWalletCreateWalletParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletCreateWalletReturns {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "address")]
    pub address: String,
}
impl Schema for BinanceWalletWalletCreateWalletReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"privateKey\":{\"type\":\"string\"},\"address\":{\"type\":\"string\"}},\"required\":[\"privateKey\",\"address\"]}")
    }
}