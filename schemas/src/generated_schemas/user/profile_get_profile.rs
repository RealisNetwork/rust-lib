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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileReturnsBanParams {
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileReturns {
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetProfileReturnsBanParams,
}
impl Schema for UserProfileGetProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isConfirmed\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"notice\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"email\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"suspicious\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"bannedUntil\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"updatedAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"ban\",\"registeredAt\"]}")
    }
}