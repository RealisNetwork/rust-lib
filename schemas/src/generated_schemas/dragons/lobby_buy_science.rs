// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyBuyScienceParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "sciencePurchaseKey")]
    pub science_purchase_key: String,
}
impl Schema for DragonsLobbyBuyScienceParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"sciencePurchaseKey\":{\"type\":\"string\"}},\"required\":[\"userId\",\"sciencePurchaseKey\"]}") . unwrap ()
    }
}
impl Agent for DragonsLobbyBuyScienceParams {
    fn topic() -> &'static str {
        "dragons_lobby_buyScience"
    }
    fn method() -> &'static str {
        "lobby_buyScience"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyBuyScienceReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyBuyScienceReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyBuyScienceReturns;
impl Schema for DragonsLobbyBuyScienceReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyBuyScienceReturns {
    fn topic() -> &'static str {
        "dragons_lobby_buyScience"
    }
    fn method() -> &'static str {
        "lobby_buyScience"
    }
    fn agent() -> &'static str {
        "dragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
