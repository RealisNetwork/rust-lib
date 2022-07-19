// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for UserProfileGetMyProfileParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(UserProfileGetMyProfileParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileGetMyProfileParams;
impl Schema for UserProfileGetMyProfileParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for UserProfileGetMyProfileParams {
    fn topic() -> &'static str {
        "user_profile_getMyProfile"
    }
    fn method() -> &'static str {
        "profile_getMyProfile"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturnsBanParams {
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturns {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetMyProfileReturnsBanParams,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
}
impl Schema for UserProfileGetMyProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isNicknameChanged\":{\"type\":\"boolean\"},\"suspicious\":{\"type\":\"boolean\"},\"notice\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"userId\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"whoBanned\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"registeredAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"ban\",\"registeredAt\"]}")
    }
}
impl Agent for UserProfileGetMyProfileReturns {
    fn topic() -> &'static str {
        "user_profile_getMyProfile"
    }
    fn method() -> &'static str {
        "profile_getMyProfile"
    }
    fn agent() -> &'static str {
        "user"
    }
}
