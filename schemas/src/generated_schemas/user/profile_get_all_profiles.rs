// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesParams {
    #[serde(rename = "page")]
    pub page: f64,
    #[serde(rename = "perPage")]
    pub per_page: f64,
}
impl Schema for UserProfileGetAllProfilesParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}") . unwrap ()
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
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturnsParams {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetAllProfilesReturnsParamsBanParams,
    #[serde(rename = "lifeTimeInterest")]
    pub life_time_interest: bool,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "lifeTimeInterestPercent")]
    pub life_time_interest_percent: String,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturns(pub Vec<UserProfileGetAllProfilesReturnsParams>);
impl Schema for UserProfileGetAllProfilesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"},\"hasPassword\":{\"type\":\"boolean\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"isNicknameChanged\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"suspicious\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"lifeTimeInterest\":{\"type\":\"boolean\"},\"isConfirmed\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"notice\":{\"type\":\"string\"},\"isDeleted\":{\"type\":\"boolean\"},\"lifeTimeInterestPercent\":{\"type\":\"string\"},\"registeredAt\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"hasPassword\",\"ban\",\"registeredAt\",\"lifeTimeInterest\",\"lifeTimeInterestPercent\"]}}")
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
