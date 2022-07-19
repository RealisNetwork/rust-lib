// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyUpgradeCardParams {
    #[serde(rename = "cardId")]
    pub card_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyUpgradeCardParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"cardId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"cardId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyUpgradeCardReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
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