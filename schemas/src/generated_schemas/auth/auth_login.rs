// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthLoginParams {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "appId")]
    pub app_id: Option<i32>,
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
}
impl Schema for AuthAuthLoginParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"username\":{\"type\":\"string\"},\"password\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"deviceId\":{\"type\":\"string\"}},\"required\":[\"username\",\"password\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthLoginReturns {
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
}
impl Schema for AuthAuthLoginReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"refresh_token\":{\"type\":\"string\"},\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"access_token\":{\"type\":\"string\"}},\"required\":[\"access_token\",\"expires_in\",\"refresh_expires_in\",\"refresh_token\",\"userId\"]}")
    }
}
