// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetUserIdByEmailParams {
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for UserProfileGetUserIdByEmailParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"}},\"required\":[\"email\"]}") . unwrap ()
    }
}
impl Agent for UserProfileGetUserIdByEmailParams {
    fn topic() -> &'static str {
        "user_profile_getUserIdByEmail"
    }
    fn method() -> &'static str {
        "profile_getUserIdByEmail"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetUserIdByEmailReturns(pub String);
impl Schema for UserProfileGetUserIdByEmailReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for UserProfileGetUserIdByEmailReturns {
    fn topic() -> &'static str {
        "user_profile_getUserIdByEmail"
    }
    fn method() -> &'static str {
        "profile_getUserIdByEmail"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
