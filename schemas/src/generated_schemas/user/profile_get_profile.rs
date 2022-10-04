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
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
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
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
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
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "lastActivity")]
    pub last_activity: String,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetProfileReturnsBanParams,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "lifeTimeInterest")]
    pub life_time_interest: bool,
    #[serde(rename = "lifeTimeInterestPercent")]
    pub life_time_interest_percent: String,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
}
impl Schema for UserProfileGetProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"suspicious\":{\"type\":\"boolean\"},\"hasPassword\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"isDeleted\":{\"type\":\"boolean\"},\"lastActivity\":{\"type\":\"string\"},\"ban\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"reason\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"notice\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"lifeTimeInterest\":{\"type\":\"boolean\"},\"lifeTimeInterestPercent\":{\"type\":\"string\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"verified\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"hasPassword\",\"ban\",\"registeredAt\",\"lastActivity\",\"lifeTimeInterest\",\"lifeTimeInterestPercent\"]}")
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
