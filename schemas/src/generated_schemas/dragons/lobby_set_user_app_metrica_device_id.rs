// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbySetUserAppMetricaDeviceIdParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appMetricaDeviceId")]
    pub app_metrica_device_id: String,
}
impl Schema for DragonsLobbySetUserAppMetricaDeviceIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appMetricaDeviceId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appMetricaDeviceId\"]}")
    }
}
impl Agent for DragonsLobbySetUserAppMetricaDeviceIdParams {
    fn topic() -> &'static str {
        "dragons_lobby_setUserAppMetricaDeviceId"
    }
    fn method() -> &'static str {
        "lobby_setUserAppMetricaDeviceId"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbySetUserAppMetricaDeviceIdReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbySetUserAppMetricaDeviceIdReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbySetUserAppMetricaDeviceIdReturns;
impl Schema for DragonsLobbySetUserAppMetricaDeviceIdReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbySetUserAppMetricaDeviceIdReturns {
    fn topic() -> &'static str {
        "dragons_lobby_setUserAppMetricaDeviceId"
    }
    fn method() -> &'static str {
        "lobby_setUserAppMetricaDeviceId"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
