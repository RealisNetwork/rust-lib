// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragonsLobbyGetBadTransactionsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetBadTransactionsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetBadTransactionsParams;
impl Schema for DragonsLobbyGetBadTransactionsParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for DragonsLobbyGetBadTransactionsParams {
    fn topic() -> &'static str {
        "dragons_lobby_getBadTransactions"
    }
    fn method() -> &'static str {
        "lobby_getBadTransactions"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetBadTransactionsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetBadTransactionsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetBadTransactionsReturns;
impl Schema for DragonsLobbyGetBadTransactionsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetBadTransactionsReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getBadTransactions"
    }
    fn method() -> &'static str {
        "lobby_getBadTransactions"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
