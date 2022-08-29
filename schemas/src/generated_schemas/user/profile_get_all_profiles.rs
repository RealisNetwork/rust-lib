// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesParams {
    #[serde(rename = "perPage")]
    pub per_page: f64,
    #[serde(rename = "page")]
    pub page: f64,
}
impl Schema for UserProfileGetAllProfilesParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}")
    }
}
impl Agent for UserProfileGetAllProfilesParams {
    fn topic() -> &'static str {
        "user_profile_getAllProfiles"
    }
    fn method() -> &'static str {
        "profile_getAllProfiles"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturnsParamsBanParams {
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturnsParams {
    #[serde(rename = "ban")]
    pub ban: UserProfileGetAllProfilesReturnsParamsBanParams,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturns(pub Vec<UserProfileGetAllProfilesReturnsParams>);
impl Schema for UserProfileGetAllProfilesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"ban\":{\"type\":\"object\",\"properties\":{\"reason\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"whoBanned\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"suspicious\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isBanned\":{\"type\":\"boolean\"},\"isConfirmed\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"notice\":{\"type\":\"string\"},\"isDeleted\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"ban\",\"registeredAt\"]}}")
    }
}
impl Agent for UserProfileGetAllProfilesReturns {
    fn topic() -> &'static str {
        "user_profile_getAllProfiles"
    }
    fn method() -> &'static str {
        "profile_getAllProfiles"
    }
    fn agent() -> &'static str {
        "user"
    }
}
