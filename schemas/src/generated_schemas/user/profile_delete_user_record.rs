// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileDeleteUserRecordParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for UserProfileDeleteUserRecordParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for UserProfileDeleteUserRecordParams {
    fn topic() -> &'static str {
        "user_profile_deleteUserRecord"
    }
    fn method() -> &'static str {
        "profile_deleteUserRecord"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileDeleteUserRecordReturns(pub bool);
impl Schema for UserProfileDeleteUserRecordReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for UserProfileDeleteUserRecordReturns {
    fn topic() -> &'static str {
        "user_profile_deleteUserRecord"
    }
    fn method() -> &'static str {
        "profile_deleteUserRecord"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
