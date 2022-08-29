// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for UserProfileGetProfileParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
impl Agent for UserProfileGetProfileParams {
    fn topic() -> &'static str {
        "user_profile_getProfile"
    }
    fn method() -> &'static str {
        "profile_getProfile"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileReturnsBanParams {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
    #[serde(rename = "reason")]
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileReturns {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetProfileReturnsBanParams,
}
impl Schema for UserProfileGetProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"notice\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isConfirmed\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"suspicious\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"updatedAt\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"ban\",\"registeredAt\"]}")
    }
}
impl Agent for UserProfileGetProfileReturns {
    fn topic() -> &'static str {
        "user_profile_getProfile"
    }
    fn method() -> &'static str {
        "profile_getProfile"
    }
    fn agent() -> &'static str {
        "user"
    }
}
