// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletGetUserIdByAddressParams {
    #[serde(rename = "address")]
    pub address: String,
}
impl Schema for BinanceWalletWalletGetUserIdByAddressParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinanceWalletWalletGetUserIdByAddressReturns(String);
