// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatusGetParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for UserStatusGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for UserStatusGetParams {
    fn topic() -> &'static str {
        "user_status_get"
    }
    fn method() -> &'static str {
        "status_get"
    }
    fn agent() -> &'static str {
        "user"
    }
}
impl<'de> Deserialize<'de> for UserStatusGetReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(UserStatusGetReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserStatusGetReturns;
impl Schema for UserStatusGetReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for UserStatusGetReturns {
    fn topic() -> &'static str {
        "user_status_get"
    }
    fn method() -> &'static str {
        "status_get"
    }
    fn agent() -> &'static str {
        "user"
    }
}
