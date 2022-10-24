// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyUseSpinParams {
    #[serde(rename = "spinTypeId")]
    pub spin_type_id: f64,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for CatsLobbyUseSpinParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"spinTypeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"spinTypeId\"]}") . unwrap ()
    }
}
impl Agent for CatsLobbyUseSpinParams {
    fn topic() -> &'static str {
        "cats_lobby_useSpin"
    }
    fn method() -> &'static str {
        "lobby_useSpin"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
impl<'de> Deserialize<'de> for CatsLobbyUseSpinReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(CatsLobbyUseSpinReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyUseSpinReturns;
impl Schema for CatsLobbyUseSpinReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyUseSpinReturns {
    fn topic() -> &'static str {
        "cats_lobby_useSpin"
    }
    fn method() -> &'static str {
        "lobby_useSpin"
    }
    fn agent() -> &'static str {
        "cats"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
