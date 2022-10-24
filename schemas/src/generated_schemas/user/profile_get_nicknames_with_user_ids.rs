// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetNicknamesWithUserIdsParams {
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
}
impl Schema for UserProfileGetNicknamesWithUserIdsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userIds\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"userIds\"]}") . unwrap ()
    }
}
impl Agent for UserProfileGetNicknamesWithUserIdsParams {
    fn topic() -> &'static str {
        "user_profile_getNicknamesWithUserIds"
    }
    fn method() -> &'static str {
        "profile_getNicknamesWithUserIds"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetNicknamesWithUserIdsReturnsParams {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetNicknamesWithUserIdsReturns(
    pub Vec<UserProfileGetNicknamesWithUserIdsReturnsParams>,
);
impl Schema for UserProfileGetNicknamesWithUserIdsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"nickname\"]}}")
    }
}
impl Agent for UserProfileGetNicknamesWithUserIdsReturns {
    fn topic() -> &'static str {
        "user_profile_getNicknamesWithUserIds"
    }
    fn method() -> &'static str {
        "profile_getNicknamesWithUserIds"
    }
    fn agent() -> &'static str {
        "user"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
