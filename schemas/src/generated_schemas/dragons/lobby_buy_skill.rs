// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragonsLobbyBuySkillParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "skillPurchaseKey")]
    pub skill_purchase_key: String,
}
impl Schema for DragonsLobbyBuySkillParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"skillPurchaseKey\":{\"type\":\"string\"}},\"required\":[\"userId\",\"skillPurchaseKey\"]}")
    }
}
impl<'de> Deserialize<'de> for DragonsLobbyBuySkillReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DragonsLobbyBuySkillReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragonsLobbyBuySkillReturns;
impl Schema for DragonsLobbyBuySkillReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}