// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthGetUserInfoByTokenParams {
    #[serde(rename = "token")]
    pub token: String,
}
impl Schema for AuthAuthGetUserInfoByTokenParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":[\"token\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthGetUserInfoByTokenParams {
    fn topic() -> &'static str {
        "auth_auth_getUserInfoByToken"
    }
    fn method() -> &'static str {
        "auth_getUserInfoByToken"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthGetUserInfoByTokenReturns {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "roles")]
    pub roles: Vec<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "emailVerified")]
    pub email_verified: bool,
    #[serde(rename = "username")]
    pub username: String,
}
impl Schema for AuthAuthGetUserInfoByTokenReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"},\"isBanned\":{\"type\":\"boolean\"},\"id\":{\"type\":\"string\"},\"roles\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"userId\":{\"type\":\"string\"},\"emailVerified\":{\"type\":\"boolean\"},\"username\":{\"type\":\"string\"}},\"required\":[\"id\",\"username\",\"emailVerified\",\"email\",\"roles\",\"isBanned\",\"userId\"]}")
    }
}
impl Agent for AuthAuthGetUserInfoByTokenReturns {
    fn topic() -> &'static str {
        "auth_auth_getUserInfoByToken"
    }
    fn method() -> &'static str {
        "auth_getUserInfoByToken"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
