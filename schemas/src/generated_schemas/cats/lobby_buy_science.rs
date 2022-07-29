// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyBuyScienceParams {
    #[serde(rename = "sciencePurchaseKey")]
    pub science_purchase_key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyBuyScienceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"sciencePurchaseKey\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"sciencePurchaseKey\"]}")
    }
}
impl Agent for CatsLobbyBuyScienceParams {
    fn topic() -> &'static str {
        "cats_lobby_buyScience"
    }
    fn method() -> &'static str {
        "lobby_buyScience"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyBuyScienceReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyBuyScienceReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyBuyScienceReturns;
impl Schema for CatsLobbyBuyScienceReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyBuyScienceReturns {
    fn topic() -> &'static str {
        "cats_lobby_buyScience"
    }
    fn method() -> &'static str {
        "lobby_buyScience"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
