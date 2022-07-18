// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for BalancesBalancesGetSoftCurrencyBalanceParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(BalancesBalancesGetSoftCurrencyBalanceParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct BalancesBalancesGetSoftCurrencyBalanceParams;
impl Schema for BalancesBalancesGetSoftCurrencyBalanceParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancesBalancesGetSoftCurrencyBalanceReturns {
    #[serde(rename = "balance")]
    pub balance: i32,
}
