// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUserValidationParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsLobbyUserValidationParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyUserValidationParams {
    fn topic() -> &'static str {
        "cats_lobby_userValidation"
    }
    fn method() -> &'static str {
        "lobby_userValidation"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyUserValidationReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyUserValidationReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyUserValidationReturns;
impl Schema for CatsLobbyUserValidationReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyUserValidationReturns {
    fn topic() -> &'static str {
        "cats_lobby_userValidation"
    }
    fn method() -> &'static str {
        "lobby_userValidation"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
