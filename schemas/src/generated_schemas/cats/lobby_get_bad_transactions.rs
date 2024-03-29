// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetBadTransactionsParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetBadTransactionsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetBadTransactionsParams;
impl Schema for CatsLobbyGetBadTransactionsParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for CatsLobbyGetBadTransactionsParams {
    fn topic() -> &'static str {
        "cats_lobby_getBadTransactions"
    }
    fn method() -> &'static str {
        "lobby_getBadTransactions"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetBadTransactionsReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetBadTransactionsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetBadTransactionsReturns;
impl Schema for CatsLobbyGetBadTransactionsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetBadTransactionsReturns {
    fn topic() -> &'static str {
        "cats_lobby_getBadTransactions"
    }
    fn method() -> &'static str {
        "lobby_getBadTransactions"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
