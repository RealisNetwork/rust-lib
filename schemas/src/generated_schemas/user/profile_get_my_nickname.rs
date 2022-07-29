// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for UserProfileGetMyNicknameParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(UserProfileGetMyNicknameParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileGetMyNicknameParams;
impl Schema for UserProfileGetMyNicknameParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for UserProfileGetMyNicknameParams {
    fn topic() -> &'static str {
        "user_profile_getMyNickname"
    }
    fn method() -> &'static str {
        "profile_getMyNickname"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyNicknameReturns(pub String);
impl Schema for UserProfileGetMyNicknameReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for UserProfileGetMyNicknameReturns {
    fn topic() -> &'static str {
        "user_profile_getMyNickname"
    }
    fn method() -> &'static str {
        "profile_getMyNickname"
    }
    fn agent() -> &'static str {
        "user"
    }
}
