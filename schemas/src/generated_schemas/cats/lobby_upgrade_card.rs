// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUpgradeCardParams {
    #[serde(rename = "cardId")]
    pub card_id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyUpgradeCardParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"cardId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"cardId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyUpgradeCardParams {
    fn topic() -> &'static str {
        "cats_lobby_upgradeCard"
    }
    fn method() -> &'static str {
        "lobby_upgradeCard"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for CatsLobbyUpgradeCardReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyUpgradeCardReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyUpgradeCardReturns;
impl Schema for CatsLobbyUpgradeCardReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyUpgradeCardReturns {
    fn topic() -> &'static str {
        "cats_lobby_upgradeCard"
    }
    fn method() -> &'static str {
        "lobby_upgradeCard"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
