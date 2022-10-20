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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturnsParamsBanParams {
    #[serde(rename = "updatedAt", deserialize_with = "deserialize_to_string")]
    pub updated_at: String,
    #[serde(rename = "bannedUntil", deserialize_with = "deserialize_to_string")]
    pub banned_until: String,
    #[serde(rename = "whoBanned", deserialize_with = "deserialize_to_string")]
    pub who_banned: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "reason", deserialize_with = "deserialize_to_string")]
    pub reason: String,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_to_string")]
    pub created_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturnsParams {
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "registeredAt", deserialize_with = "deserialize_to_string")]
    pub registered_at: String,
    #[serde(rename = "email", deserialize_with = "deserialize_to_string")]
    pub email: String,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(
        rename = "lifeTimeInterestPercent",
        deserialize_with = "deserialize_to_string"
    )]
    pub life_time_interest_percent: String,
    #[serde(rename = "lifeTimeInterest")]
    pub life_time_interest: bool,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetAllProfilesReturnsParamsBanParams,
    #[serde(rename = "notice", deserialize_with = "deserialize_to_string")]
    pub notice: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
    #[serde(rename = "nickname", deserialize_with = "deserialize_to_string")]
    pub nickname: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetAllProfilesReturns(pub Vec<UserProfileGetAllProfilesReturnsParams>);
impl Schema for UserProfileGetAllProfilesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isNicknameChanged\":{\"type\":\"boolean\"},\"registeredAt\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"isConfirmed\":{\"type\":\"boolean\"},\"lifeTimeInterestPercent\":{\"type\":\"string\"},\"lifeTimeInterest\":{\"type\":\"boolean\"},\"isSubscribedToMailing\":{\"type\":\"boolean\"},\"suspicious\":{\"type\":\"boolean\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"verified\":{\"type\":\"boolean\"},\"ban\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"string\"},\"bannedUntil\":{\"type\":\"string\"},\"whoBanned\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"reason\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"reason\",\"whoBanned\",\"createdAt\",\"updatedAt\",\"bannedUntil\"]},\"notice\":{\"type\":\"string\"},\"isDeleted\":{\"type\":\"boolean\"},\"hasPassword\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"userId\",\"nickname\",\"email\",\"verified\",\"isNicknameChanged\",\"isSubscribedToMailing\",\"suspicious\",\"isConfirmed\",\"notice\",\"isBanned\",\"isDeleted\",\"hasPassword\",\"ban\",\"registeredAt\",\"lifeTimeInterest\",\"lifeTimeInterestPercent\"]}}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
