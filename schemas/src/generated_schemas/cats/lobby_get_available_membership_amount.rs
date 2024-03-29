// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetAvailableMembershipAmountParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetAvailableMembershipAmountParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetAvailableMembershipAmountParams;
impl Schema for CatsLobbyGetAvailableMembershipAmountParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for CatsLobbyGetAvailableMembershipAmountParams {
    fn topic() -> &'static str {
        "cats_lobby_getAvailableMembershipAmount"
    }
    fn method() -> &'static str {
        "lobby_getAvailableMembershipAmount"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetAvailableMembershipAmountReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetAvailableMembershipAmountReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetAvailableMembershipAmountReturns;
impl Schema for CatsLobbyGetAvailableMembershipAmountReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetAvailableMembershipAmountReturns {
    fn topic() -> &'static str {
        "cats_lobby_getAvailableMembershipAmount"
    }
    fn method() -> &'static str {
        "lobby_getAvailableMembershipAmount"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
