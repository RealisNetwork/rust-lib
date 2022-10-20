// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for UserProfileGetMyProfileParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(UserProfileGetMyProfileParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct UserProfileGetMyProfileParams;
impl Schema for UserProfileGetMyProfileParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturnsBanParams {
    #[serde(rename = "bannedUntil", deserialize_with = "deserialize_to_string")]
    pub banned_until: String,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_to_string")]
    pub created_at: String,
    #[serde(rename = "whoBanned", deserialize_with = "deserialize_to_string")]
    pub who_banned: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "reason", deserialize_with = "deserialize_to_string")]
    pub reason: String,
    #[serde(rename = "updatedAt", deserialize_with = "deserialize_to_string")]
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturns {
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetMyProfileReturnsBanParams,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "email", deserialize_with = "deserialize_to_string")]
    pub email: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "lifeTimeInterest")]
    pub life_time_interest: bool,
    #[serde(rename = "nickname", deserialize_with = "deserialize_to_string")]
    pub nickname: String,
    #[serde(rename = "notice", deserialize_with = "deserialize_to_string")]
    pub notice: String,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(
        rename = "lifeTimeInterestPercent",
        deserialize_with = "deserialize_to_string"
    )]
    pub life_time_interest_percent: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "registeredAt", deserialize_with = "deserialize_to_string")]
    pub registered_at: String,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "lastActivity", deserialize_with = "deserialize_to_string")]
    pub last_activity: String,
}
impl Schema for UserProfileGetMyProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isBanned\":{\"type\":\"boolean\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"bannedUntil\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"userId\":{\"type\":\"string\"},\"isDeleted\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isNicknameChanged\":{\"type\":\"boolean\"},\"lifeTimeInterest\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"notice\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"hasPassword\":{\"type\":\"boolean\"},\"lifeTimeInterestPercent\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"suspicious\":{\"type\":\"boolean\"},\"lastActivity\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"hasPassword\",\"ban\",\"registeredAt\",\"lastActivity\",\"lifeTimeInterest\",\"lifeTimeInterestPercent\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
