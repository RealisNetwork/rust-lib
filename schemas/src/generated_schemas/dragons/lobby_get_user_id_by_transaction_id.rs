// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyGetUserIdByTransactionIdParams {
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}
impl Schema for DragonsLobbyGetUserIdByTransactionIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"transactionId\":{\"type\":\"string\"}},\"required\":[\"transactionId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetUserIdByTransactionIdReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyGetUserIdByTransactionIdReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetUserIdByTransactionIdReturns;
impl Schema for DragonsLobbyGetUserIdByTransactionIdReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
