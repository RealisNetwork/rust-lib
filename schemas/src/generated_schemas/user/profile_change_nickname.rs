// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileChangeNicknameParams {
    #[serde(rename = "newNickname", deserialize_with = "deserialize_to_string")]
    pub new_nickname: String,
}
impl Schema for UserProfileChangeNicknameParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"newNickname\":{\"type\":\"string\"}},\"required\":[\"newNickname\"]}") . unwrap ()
    }
}
impl Agent for UserProfileChangeNicknameParams {
    fn topic() -> &'static str {
        "user_profile_changeNickname"
    }
    fn method() -> &'static str {
        "profile_changeNickname"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileChangeNicknameReturns(pub bool);
impl Schema for UserProfileChangeNicknameReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for UserProfileChangeNicknameReturns {
    fn topic() -> &'static str {
        "user_profile_changeNickname"
    }
    fn method() -> &'static str {
        "profile_changeNickname"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
