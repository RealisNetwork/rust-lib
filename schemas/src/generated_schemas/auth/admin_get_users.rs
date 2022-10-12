// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminGetUsersParams {
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
}
impl Schema for AuthAdminGetUsersParams {
    fn schema() -> Value {
        serde_json::from_str(
            "{\"type\":\"object\",\"properties\":{\"isActive\":{\"type\":\"boolean\"}}}",
        )
        .unwrap()
    }
}
impl Agent for AuthAdminGetUsersParams {
    fn topic() -> &'static str {
        "auth_admin_getUsers"
    }
    fn method() -> &'static str {
        "admin_getUsers"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminGetUsersReturnsParams {
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    #[serde(rename = "role")]
    pub role: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "googleId")]
    pub google_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "appIds")]
    pub app_ids: Vec<f64>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "facebookId")]
    pub facebook_id: String,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAdminGetUsersReturns(pub Vec<AuthAdminGetUsersReturnsParams>);
impl Schema for AuthAdminGetUsersReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"passwordHash\":{\"type\":\"string\"},\"role\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"googleId\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"appIds\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"createdAt\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"},\"facebookId\":{\"type\":\"string\"},\"IsActive\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"email\",\"userId\",\"passwordHash\",\"isBanned\",\"IsActive\",\"googleId\",\"facebookId\",\"appIds\",\"createdAt\",\"updatedAt\",\"role\"]}}")
    }
}
impl Agent for AuthAdminGetUsersReturns {
    fn topic() -> &'static str {
        "auth_admin_getUsers"
    }
    fn method() -> &'static str {
        "admin_getUsers"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
