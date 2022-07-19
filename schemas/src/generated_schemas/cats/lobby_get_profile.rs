// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetProfileParams {
    #[serde(rename = "deviceId")]
    pub device_id: i64,
}
impl Schema for CatsLobbyGetProfileParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"deviceId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"deviceId\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetProfileReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyGetProfileReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetProfileReturns;
impl Schema for CatsLobbyGetProfileReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
