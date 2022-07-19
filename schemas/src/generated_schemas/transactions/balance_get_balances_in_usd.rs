// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for TransactionsBalanceGetBalancesInUsdParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(TransactionsBalanceGetBalancesInUsdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TransactionsBalanceGetBalancesInUsdParams;
impl Schema for TransactionsBalanceGetBalancesInUsdParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsBalanceGetBalancesInUsdReturns(i64);
impl Schema for TransactionsBalanceGetBalancesInUsdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}")
    }
}
