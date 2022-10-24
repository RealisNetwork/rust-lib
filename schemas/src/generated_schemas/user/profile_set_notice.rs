// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileSetNoticeParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "notice", deserialize_with = "deserialize_to_string")]
    pub notice: String,
}
impl Schema for UserProfileSetNoticeParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"notice\":{\"type\":\"string\"}},\"required\":[\"userId\",\"notice\"]}") . unwrap ()
    }
}
impl Agent for UserProfileSetNoticeParams {
    fn topic() -> &'static str {
        "user_profile_setNotice"
    }
    fn method() -> &'static str {
        "profile_setNotice"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileSetNoticeReturns(pub bool);
impl Schema for UserProfileSetNoticeReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for UserProfileSetNoticeReturns {
    fn topic() -> &'static str {
        "user_profile_setNotice"
    }
    fn method() -> &'static str {
        "profile_setNotice"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
