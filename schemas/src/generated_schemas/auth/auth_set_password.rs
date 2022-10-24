// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSetPasswordParams {
    #[serde(rename = "passwordHash", deserialize_with = "deserialize_to_string")]
    pub password_hash: String,
    #[serde(rename = "password", deserialize_with = "deserialize_to_string")]
    pub password: String,
    #[serde(rename = "providerId")]
    pub provider_id: Option<String>,
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
}
impl Schema for AuthAuthSetPasswordParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"passwordHash\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"providerId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"passwordHash\",\"password\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthSetPasswordParams {
    fn topic() -> &'static str {
        "auth_auth_setPassword"
    }
    fn method() -> &'static str {
        "auth_setPassword"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSetPasswordReturns {
    #[serde(rename = "access_token", deserialize_with = "deserialize_to_string")]
    pub access_token: String,
    #[serde(rename = "id_token", deserialize_with = "deserialize_to_string")]
    pub id_token: String,
    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: i32,
    #[serde(rename = "refresh_token", deserialize_with = "deserialize_to_string")]
    pub refresh_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for AuthAuthSetPasswordReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"access_token\":{\"type\":\"string\"},\"id_token\":{\"type\":\"string\"},\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"refresh_token\":{\"type\":\"string\"},\"expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"access_token\",\"expires_in\",\"id_token\",\"refresh_expires_in\",\"refresh_token\",\"userId\"]}")
    }
}
impl Agent for AuthAuthSetPasswordReturns {
    fn topic() -> &'static str {
        "auth_auth_setPassword"
    }
    fn method() -> &'static str {
        "auth_setPassword"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
