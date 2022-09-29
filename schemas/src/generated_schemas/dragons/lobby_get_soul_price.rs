// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragonsLobbyGetSoulPriceParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetSoulPriceParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetSoulPriceParams;
impl Schema for DragonsLobbyGetSoulPriceParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for DragonsLobbyGetSoulPriceParams {
    fn topic() -> &'static str {
        "dragons_lobby_getSoulPrice"
    }
    fn method() -> &'static str {
        "lobby_getSoulPrice"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyGetSoulPriceReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyGetSoulPriceReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyGetSoulPriceReturns;
impl Schema for DragonsLobbyGetSoulPriceReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyGetSoulPriceReturns {
    fn topic() -> &'static str {
        "dragons_lobby_getSoulPrice"
    }
    fn method() -> &'static str {
        "lobby_getSoulPrice"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
