// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyBuySkillParams {
    #[serde(rename = "skillPurchaseKey")]
    pub skill_purchase_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyBuySkillParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"skillPurchaseKey\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"skillPurchaseKey\"]}")
    }
}
impl Agent for CatsLobbyBuySkillParams {
    fn topic() -> &'static str {
        "cats_lobby_buySkill"
    }
    fn method() -> &'static str {
        "lobby_buySkill"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyBuySkillReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyBuySkillReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyBuySkillReturns;
impl Schema for CatsLobbyBuySkillReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyBuySkillReturns {
    fn topic() -> &'static str {
        "cats_lobby_buySkill"
    }
    fn method() -> &'static str {
        "lobby_buySkill"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
