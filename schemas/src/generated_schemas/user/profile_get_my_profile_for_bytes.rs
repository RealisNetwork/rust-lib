// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for UserProfileGetMyProfileForBytesParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(UserProfileGetMyProfileForBytesParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileGetMyProfileForBytesParams;
impl Schema for UserProfileGetMyProfileForBytesParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for UserProfileGetMyProfileForBytesParams {
    fn topic() -> &'static str {
        "user_profile_getMyProfileForBytes"
    }
    fn method() -> &'static str {
        "profile_getMyProfileForBytes"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileForBytesReturns {
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
}
impl Schema for UserProfileGetMyProfileForBytesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"isConfirmed\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"},\"notice\":{\"type\":\"string\"},\"isDeleted\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"reason\":{\"type\":\"string\"},\"registeredAt\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"suspicious\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"reason\",\"registeredAt\"]}")
    }
}
impl Agent for UserProfileGetMyProfileForBytesReturns {
    fn topic() -> &'static str {
        "user_profile_getMyProfileForBytes"
    }
    fn method() -> &'static str {
        "profile_getMyProfileForBytes"
    }
    fn agent() -> &'static str {
        "user"
    }
}
