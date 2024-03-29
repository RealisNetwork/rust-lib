// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminUserRoleGetAllUsersWithNicknameParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(AdminUserRoleGetAllUsersWithNicknameParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminUserRoleGetAllUsersWithNicknameParams;
impl Schema for AdminUserRoleGetAllUsersWithNicknameParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for AdminUserRoleGetAllUsersWithNicknameParams {
    fn topic() -> &'static str {
        "admin_userRole_getAllUsersWithNickname"
    }
    fn method() -> &'static str {
        "userRole_getAllUsersWithNickname"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllUsersWithNicknameReturnsParams {
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllUsersWithNicknameReturns(
    pub Vec<AdminUserRoleGetAllUsersWithNicknameReturnsParams>,
);
impl Schema for AdminUserRoleGetAllUsersWithNicknameReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"role\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"nickname\",\"role\"]}}")
    }
}
impl Agent for AdminUserRoleGetAllUsersWithNicknameReturns {
    fn topic() -> &'static str {
        "admin_userRole_getAllUsersWithNickname"
    }
    fn method() -> &'static str {
        "userRole_getAllUsersWithNickname"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
