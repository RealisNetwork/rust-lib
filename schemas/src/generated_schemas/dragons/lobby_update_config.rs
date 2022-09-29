// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUpdateConfigParams {
    #[serde(rename = "configKey")]
    pub config_key: String,
    #[serde(rename = "configJson")]
    pub config_json: String,
}
impl Schema for DragonsLobbyUpdateConfigParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"configKey\":{\"type\":\"string\"},\"configJson\":{\"type\":\"string\"}},\"required\":[\"configKey\",\"configJson\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyUpdateConfigParams {
    fn topic() -> &'static str {
        "dragons_lobby_updateConfig"
    }
    fn method() -> &'static str {
        "lobby_updateConfig"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUpdateConfigReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyUpdateConfigReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyUpdateConfigReturns;
impl Schema for DragonsLobbyUpdateConfigReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyUpdateConfigReturns {
    fn topic() -> &'static str {
        "dragons_lobby_updateConfig"
    }
    fn method() -> &'static str {
        "lobby_updateConfig"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
