// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbySettingsSetParams {
    #[serde(rename = "quality")]
    pub quality: i8,
    #[serde(rename = "language")]
    pub language: i8,
    #[serde(rename = "HFREffects")]
    pub hfr_effects: i8,
    #[serde(rename = "sounds")]
    pub sounds: bool,
    #[serde(rename = "music")]
    pub music: bool,
}
impl Schema for LobbySettingsSetParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"quality\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"language\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"HFREffects\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"sounds\":{\"type\":\"boolean\"},\"music\":{\"type\":\"boolean\"}},\"required\":[\"sounds\",\"music\",\"quality\",\"HFREffects\",\"language\"]}")
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
}
impl<'de> Deserialize<'de> for LobbySettingsSetReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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
}
