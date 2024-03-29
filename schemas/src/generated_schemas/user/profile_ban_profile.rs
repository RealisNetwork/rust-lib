// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileBanProfileParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "reason")]
    pub reason: String,
}
impl Schema for UserProfileBanProfileParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"}},\"required\":[\"userId\",\"reason\"]}") . unwrap ()
    }
}
impl Agent for UserProfileBanProfileParams {
    fn topic() -> &'static str {
        "user_profile_banProfile"
    }
    fn method() -> &'static str {
        "profile_banProfile"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileBanProfileReturns(pub bool);
impl Schema for UserProfileBanProfileReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for UserProfileBanProfileReturns {
    fn topic() -> &'static str {
        "user_profile_banProfile"
    }
    fn method() -> &'static str {
        "profile_banProfile"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
