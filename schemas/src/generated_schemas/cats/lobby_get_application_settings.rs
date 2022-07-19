// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for CatsLobbyGetApplicationSettingsParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetApplicationSettingsParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetApplicationSettingsParams;
impl Schema for CatsLobbyGetApplicationSettingsParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetApplicationSettingsParams {
    fn topic() -> &'static str {
        "cats_lobby_getApplicationSettings"
    }
    fn method() -> &'static str {
        "lobby_getApplicationSettings"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetApplicationSettingsReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetApplicationSettingsReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetApplicationSettingsReturns;
impl Schema for CatsLobbyGetApplicationSettingsReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetApplicationSettingsReturns {
    fn topic() -> &'static str {
        "cats_lobby_getApplicationSettings"
    }
    fn method() -> &'static str {
        "lobby_getApplicationSettings"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
