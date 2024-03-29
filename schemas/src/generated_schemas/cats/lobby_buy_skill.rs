// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyBuySkillParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "skillPurchaseKey")]
    pub skill_purchase_key: String,
}
impl Schema for CatsLobbyBuySkillParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"skillPurchaseKey\":{\"type\":\"string\"}},\"required\":[\"userId\",\"skillPurchaseKey\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
