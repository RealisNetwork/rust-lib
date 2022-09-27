// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbySettingsGetParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbySettingsGetParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbySettingsGetParams;
impl Schema for LobbySettingsGetParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for LobbySettingsGetParams {
    fn topic() -> &'static str {
        "lobby_settings_get"
    }
    fn method() -> &'static str {
        "settings_get"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbySettingsGetReturns {
    #[serde(rename = "language")]
    pub language: i8,
    #[serde(rename = "quality")]
    pub quality: i8,
    #[serde(rename = "HFREffects")]
    pub hfr_effects: i8,
    #[serde(rename = "music")]
    pub music: bool,
    #[serde(rename = "sounds")]
    pub sounds: bool,
    #[serde(rename = "isChanged")]
    pub is_changed: bool,
}
impl Schema for LobbySettingsGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"language\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"quality\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"HFREffects\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"music\":{\"type\":\"boolean\"},\"sounds\":{\"type\":\"boolean\"},\"isChanged\":{\"type\":\"boolean\"}},\"required\":[\"music\",\"sounds\",\"language\",\"quality\",\"HFREffects\",\"isChanged\"]}")
    }
}
impl Agent for LobbySettingsGetReturns {
    fn topic() -> &'static str {
        "lobby_settings_get"
    }
    fn method() -> &'static str {
        "settings_get"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
