// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbySetReferrerParams {
    #[serde(rename = "siteReferrerId")]
    pub site_referrer_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbySetReferrerParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"siteReferrerId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"siteReferrerId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbySetReferrerParams {
    fn topic() -> &'static str {
        "cats_lobby_setReferrer"
    }
    fn method() -> &'static str {
        "lobby_setReferrer"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbySetReferrerReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbySetReferrerReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbySetReferrerReturns;
impl Schema for CatsLobbySetReferrerReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbySetReferrerReturns {
    fn topic() -> &'static str {
        "cats_lobby_setReferrer"
    }
    fn method() -> &'static str {
        "lobby_setReferrer"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
