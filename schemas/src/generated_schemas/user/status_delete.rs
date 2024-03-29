// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusDeleteParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for UserStatusDeleteParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for UserStatusDeleteParams {
    fn topic() -> &'static str {
        "user_status_delete"
    }
    fn method() -> &'static str {
        "status_delete"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
impl<'de> Deserialize<'de> for UserStatusDeleteReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(UserStatusDeleteReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserStatusDeleteReturns;
impl Schema for UserStatusDeleteReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for UserStatusDeleteReturns {
    fn topic() -> &'static str {
        "user_status_delete"
    }
    fn method() -> &'static str {
        "status_delete"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
