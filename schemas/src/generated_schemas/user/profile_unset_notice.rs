// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUnsetNoticeParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for UserProfileUnsetNoticeParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for UserProfileUnsetNoticeParams {
    fn topic() -> &'static str {
        "user_profile_unsetNotice"
    }
    fn method() -> &'static str {
        "profile_unsetNotice"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUnsetNoticeReturns(pub bool);
impl Schema for UserProfileUnsetNoticeReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for UserProfileUnsetNoticeReturns {
    fn topic() -> &'static str {
        "user_profile_unsetNotice"
    }
    fn method() -> &'static str {
        "profile_unsetNotice"
    }
    fn agent() -> &'static str {
        "user"
    }
}
