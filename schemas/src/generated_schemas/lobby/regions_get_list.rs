// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyRegionsGetListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyRegionsGetListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyRegionsGetListParams;
impl Schema for LobbyRegionsGetListParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyRegionsGetListParams {
    fn topic() -> &'static str {
        "lobby_regions_getList"
    }
    fn method() -> &'static str {
        "regions_getList"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyRegionsGetListReturnsListParamsParams {
    #[serde(rename = "host", deserialize_with = "deserialize_to_string")]
    pub host: String,
    #[serde(rename = "name", deserialize_with = "deserialize_to_string")]
    pub name: String,
    #[serde(rename = "code", deserialize_with = "deserialize_to_string")]
    pub code: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyRegionsGetListReturns {
    #[serde(rename = "list")]
    pub list: Vec<LobbyRegionsGetListReturnsListParamsParams>,
}
impl Schema for LobbyRegionsGetListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"list\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"host\":{\"type\":\"string\"},\"name\":{\"type\":\"string\"},\"code\":{\"type\":\"string\"}},\"required\":[\"name\",\"host\",\"code\"]}}},\"required\":[\"list\"]}")
    }
}
impl Agent for LobbyRegionsGetListReturns {
    fn topic() -> &'static str {
        "lobby_regions_getList"
    }
    fn method() -> &'static str {
        "regions_getList"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
