// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbySetUserAppMetricaDeviceIdParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appMetricaDeviceId")]
    pub app_metrica_device_id: String,
}
impl Schema for CatsLobbySetUserAppMetricaDeviceIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appMetricaDeviceId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appMetricaDeviceId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbySetUserAppMetricaDeviceIdParams {
    fn topic() -> &'static str {
        "cats_lobby_setUserAppMetricaDeviceId"
    }
    fn method() -> &'static str {
        "lobby_setUserAppMetricaDeviceId"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbySetUserAppMetricaDeviceIdReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbySetUserAppMetricaDeviceIdReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbySetUserAppMetricaDeviceIdReturns;
impl Schema for CatsLobbySetUserAppMetricaDeviceIdReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbySetUserAppMetricaDeviceIdReturns {
    fn topic() -> &'static str {
        "cats_lobby_setUserAppMetricaDeviceId"
    }
    fn method() -> &'static str {
        "lobby_setUserAppMetricaDeviceId"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
