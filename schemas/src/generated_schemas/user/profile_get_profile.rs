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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileReturnsBanParams {
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetProfileReturns {
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "lastActivity")]
    pub last_activity: String,
    #[serde(rename = "lifeTimeInterest")]
    pub life_time_interest: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetProfileReturnsBanParams,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "lifeTimeInterestPercent")]
    pub life_time_interest_percent: String,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
}
impl Schema for UserProfileGetProfileReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"hasPassword\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"lastActivity\":{\"type\":\"string\"},\"lifeTimeInterest\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"nickname\":{\"type\":\"string\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"isBanned\":{\"type\":\"boolean\"},\"isDeleted\":{\"type\":\"boolean\"},\"userId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"verified\":{\"type\":\"boolean\"},\"lifeTimeInterestPercent\":{\"type\":\"string\"},\"notice\":{\"type\":\"string\"},\"suspicious\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"hasPassword\",\"ban\",\"registeredAt\",\"lastActivity\",\"lifeTimeInterest\",\"lifeTimeInterestPercent\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
