// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUpdateConfigParams {
    #[serde(rename = "configKey")]
    pub config_key: String,
    #[serde(rename = "configJson")]
    pub config_json: String,
}
impl Schema for CatsLobbyUpdateConfigParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"configKey\":{\"type\":\"string\"},\"configJson\":{\"type\":\"string\"}},\"required\":[\"configKey\",\"configJson\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyUpdateConfigParams {
    fn topic() -> &'static str {
        "cats_lobby_updateConfig"
    }
    fn method() -> &'static str {
        "lobby_updateConfig"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyUpdateConfigReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyUpdateConfigReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyUpdateConfigReturns;
impl Schema for CatsLobbyUpdateConfigReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyUpdateConfigReturns {
    fn topic() -> &'static str {
        "cats_lobby_updateConfig"
    }
    fn method() -> &'static str {
        "lobby_updateConfig"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
