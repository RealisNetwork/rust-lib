// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyOpenLootboxParams {
    #[serde(rename = "lootboxId")]
    pub lootbox_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyOpenLootboxParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"lootboxId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyOpenLootboxReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyOpenLootboxReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyOpenLootboxReturns;
impl Schema for DragonsLobbyOpenLootboxReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
