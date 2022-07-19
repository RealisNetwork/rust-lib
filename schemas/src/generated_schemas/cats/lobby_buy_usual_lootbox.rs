// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyBuyUsualLootboxParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "lootboxId")]
    pub lootbox_id: i64,
}
impl Schema for CatsLobbyBuyUsualLootboxParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"lootboxId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"lootboxId\"]}")
    }
}
impl<'de> Deserialize<'de> for CatsLobbyBuyUsualLootboxReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyBuyUsualLootboxReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyBuyUsualLootboxReturns;
impl Schema for CatsLobbyBuyUsualLootboxReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
