// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for AdminUserRoleGetAllUsersWithNicknameParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(AdminUserRoleGetAllUsersWithNicknameParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct AdminUserRoleGetAllUsersWithNicknameParams;
impl Schema for AdminUserRoleGetAllUsersWithNicknameParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllUsersWithNicknameReturnsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "role")]
    pub role: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminUserRoleGetAllUsersWithNicknameReturns(
    Vec<AdminUserRoleGetAllUsersWithNicknameReturnsParams>,
);
impl Schema for AdminUserRoleGetAllUsersWithNicknameReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"role\":{\"type\":\"string\"}},\"required\":[\"userId\",\"nickname\",\"role\"]}}")
    }
}
