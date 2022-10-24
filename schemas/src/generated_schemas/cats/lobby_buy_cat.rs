// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyBuyCatParams {
    #[serde(rename = "catId")]
    pub cat_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "price")]
    pub price: f64,
}
impl Schema for CatsLobbyBuyCatParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"catId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"catId\",\"price\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyBuyCatParams {
    fn topic() -> &'static str {
        "cats_lobby_buyCat"
    }
    fn method() -> &'static str {
        "lobby_buyCat"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for CatsLobbyBuyCatReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyBuyCatReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyBuyCatReturns;
impl Schema for CatsLobbyBuyCatReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyBuyCatReturns {
    fn topic() -> &'static str {
        "cats_lobby_buyCat"
    }
    fn method() -> &'static str {
        "lobby_buyCat"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
