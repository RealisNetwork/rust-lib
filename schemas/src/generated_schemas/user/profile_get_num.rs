// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for UserProfileGetNumParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(UserProfileGetNumParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileGetNumParams;
impl Schema for UserProfileGetNumParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for UserProfileGetNumParams {
    fn topic() -> &'static str {
        "user_profile_getNum"
    }
    fn method() -> &'static str {
        "profile_getNum"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetNumReturns(pub f64);
impl Schema for UserProfileGetNumReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}")
    }
}
impl Agent for UserProfileGetNumReturns {
    fn topic() -> &'static str {
        "user_profile_getNum"
    }
    fn method() -> &'static str {
        "profile_getNum"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
