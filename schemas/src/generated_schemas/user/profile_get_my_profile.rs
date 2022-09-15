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
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "reason")]
    pub reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturns {
    #[serde(rename = "ban")]
    pub ban: UserProfileGetMyProfileReturnsBanParams,
    #[serde(rename = "lifeTimeInterestPercent")]
    pub life_time_interest_percent: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "lastActivity")]
    pub last_activity: String,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "lifeTimeInterest")]
    pub life_time_interest: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for UserProfileGetMyProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"ban\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"bannedUntil\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"lifeTimeInterestPercent\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"email\":{\"type\":\"string\"},\"notice\":{\"type\":\"string\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"lastActivity\":{\"type\":\"string\"},\"hasPassword\":{\"type\":\"boolean\"},\"suspicious\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"verified\":{\"type\":\"boolean\"},\"lifeTimeInterest\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"hasPassword\",\"ban\",\"registeredAt\",\"lastActivity\",\"lifeTimeInterest\",\"lifeTimeInterestPercent\"]}")
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
