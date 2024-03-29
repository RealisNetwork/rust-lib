// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileSetSuspiciousParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for UserProfileSetSuspiciousParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for UserProfileSetSuspiciousParams {
    fn topic() -> &'static str {
        "user_profile_setSuspicious"
    }
    fn method() -> &'static str {
        "profile_setSuspicious"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileSetSuspiciousReturns(pub bool);
impl Schema for UserProfileSetSuspiciousReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for UserProfileSetSuspiciousReturns {
    fn topic() -> &'static str {
        "user_profile_setSuspicious"
    }
    fn method() -> &'static str {
        "profile_setSuspicious"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
