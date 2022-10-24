// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbySettingsSetParams {
    #[serde(rename = "sounds")]
    pub sounds: bool,
    #[serde(rename = "HFREffects")]
    pub hfr_effects: i8,
    #[serde(rename = "quality")]
    pub quality: i8,
    #[serde(rename = "music")]
    pub music: bool,
    #[serde(rename = "language")]
    pub language: i8,
}
impl Schema for LobbySettingsSetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"sounds\":{\"type\":\"boolean\"},\"HFREffects\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"quality\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"music\":{\"type\":\"boolean\"},\"language\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"sounds\",\"music\",\"quality\",\"HFREffects\",\"language\"]}") . unwrap ()
    }
}
impl Agent for LobbySettingsSetParams {
    fn topic() -> &'static str {
        "lobby_settings_set"
    }
    fn method() -> &'static str {
        "settings_set"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for LobbySettingsSetReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbySettingsSetReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbySettingsSetReturns;
impl Schema for LobbySettingsSetReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for LobbySettingsSetReturns {
    fn topic() -> &'static str {
        "lobby_settings_set"
    }
    fn method() -> &'static str {
        "settings_set"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
