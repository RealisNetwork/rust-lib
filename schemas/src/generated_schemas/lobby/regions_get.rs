// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyRegionsGetParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyRegionsGetParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyRegionsGetParams;
impl Schema for LobbyRegionsGetParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyRegionsGetParams {
    fn topic() -> &'static str {
        "lobby_regions_get"
    }
    fn method() -> &'static str {
        "regions_get"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyRegionsGetReturnsListParamsParams {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "host")]
    pub host: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyRegionsGetReturns {
    #[serde(rename = "isFixed")]
    pub is_fixed: bool,
    #[serde(rename = "list")]
    pub list: Vec<LobbyRegionsGetReturnsListParamsParams>,
    #[serde(rename = "active")]
    pub active: String,
}
impl Schema for LobbyRegionsGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isFixed\":{\"type\":\"boolean\"},\"list\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"name\":{\"type\":\"string\"},\"host\":{\"type\":\"string\"}},\"required\":[\"name\",\"host\"]}},\"active\":{\"type\":\"string\"}},\"required\":[\"active\",\"isFixed\",\"list\"]}")
    }
}
impl Agent for LobbyRegionsGetReturns {
    fn topic() -> &'static str {
        "lobby_regions_get"
    }
    fn method() -> &'static str {
        "regions_get"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
