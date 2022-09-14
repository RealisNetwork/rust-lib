// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUpgradeCardParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "cardId")]
    pub card_id: f64,
}
impl Schema for DragonsLobbyUpgradeCardParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"cardId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"cardId\"]}")
    }
}
impl Agent for DragonsLobbyUpgradeCardParams {
    fn topic() -> &'static str {
        "dragons_lobby_upgradeCard"
    }
    fn method() -> &'static str {
        "lobby_upgradeCard"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUpgradeCardReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragonsLobbyUpgradeCardReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyUpgradeCardReturns;
impl Schema for DragonsLobbyUpgradeCardReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragonsLobbyUpgradeCardReturns {
    fn topic() -> &'static str {
        "dragons_lobby_upgradeCard"
    }
    fn method() -> &'static str {
        "lobby_upgradeCard"
    }
    fn agent() -> &'static str {
        "dragons"
    }
}
