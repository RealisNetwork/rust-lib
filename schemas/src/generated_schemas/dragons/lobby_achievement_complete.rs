// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyAchievementCompleteParams {
    #[serde(rename = "achievementKey")]
    pub achievement_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragonsLobbyAchievementCompleteParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"achievementKey\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"achievementKey\",\"userId\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyAchievementCompleteReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyAchievementCompleteReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyAchievementCompleteReturns;
impl Schema for DragonsLobbyAchievementCompleteReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
