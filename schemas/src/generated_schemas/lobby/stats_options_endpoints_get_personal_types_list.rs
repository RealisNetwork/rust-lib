// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyStatsOptionsEndpointsGetPersonalTypesListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyStatsOptionsEndpointsGetPersonalTypesListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyStatsOptionsEndpointsGetPersonalTypesListParams;
impl Schema for LobbyStatsOptionsEndpointsGetPersonalTypesListParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyStatsOptionsEndpointsGetPersonalTypesListParams {
    fn topic() -> &'static str {
        "lobby_statsOptionsEndpoints_getPersonalTypesList"
    }
    fn method() -> &'static str {
        "statsOptionsEndpoints_getPersonalTypesList"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetPersonalTypesListReturns {
    #[serde(rename = "personalTypes")]
    pub personal_types: Vec<String>,
}
impl Schema for LobbyStatsOptionsEndpointsGetPersonalTypesListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalTypes\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"personalTypes\"]}")
    }
}
impl Agent for LobbyStatsOptionsEndpointsGetPersonalTypesListReturns {
    fn topic() -> &'static str {
        "lobby_statsOptionsEndpoints_getPersonalTypesList"
    }
    fn method() -> &'static str {
        "statsOptionsEndpoints_getPersonalTypesList"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
