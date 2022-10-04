// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyGetUsersInappPurchasesParams {
    #[serde(rename = "endDate")]
    pub end_date: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "startDate")]
    pub start_date: f64,
}
impl Schema for CatsLobbyGetUsersInappPurchasesParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"endDate\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"startDate\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"startDate\",\"endDate\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyGetUsersInappPurchasesParams {
    fn topic() -> &'static str {
        "cats_lobby_getUsersInappPurchases"
    }
    fn method() -> &'static str {
        "lobby_getUsersInappPurchases"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyGetUsersInappPurchasesReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyGetUsersInappPurchasesReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyGetUsersInappPurchasesReturns;
impl Schema for CatsLobbyGetUsersInappPurchasesReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyGetUsersInappPurchasesReturns {
    fn topic() -> &'static str {
        "cats_lobby_getUsersInappPurchases"
    }
    fn method() -> &'static str {
        "lobby_getUsersInappPurchases"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
