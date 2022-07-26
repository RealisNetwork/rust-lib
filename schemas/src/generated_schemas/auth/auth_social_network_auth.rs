// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSocialNetworkAuthParams {
    #[serde(rename = "provider")]
    pub provider: String,
    #[serde(rename = "credential")]
    pub credential: String,
}
impl Schema for AuthAuthSocialNetworkAuthParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"provider\":{\"type\":\"string\"},\"credential\":{\"type\":\"string\"}},\"required\":[\"credential\",\"provider\"]}")
    }
}
impl Agent for AuthAuthSocialNetworkAuthParams {
    fn topic() -> &'static str {
        "auth_auth_socialNetworkAuth"
    }
    fn method() -> &'static str {
        "auth_socialNetworkAuth"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthSocialNetworkAuthReturns {
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: Option<i32>,
    #[serde(rename = "refresh_token")]
    pub refresh_token: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AuthAuthSocialNetworkAuthReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"access_token\":{\"type\":\"string\"},\"refresh_expires_in\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"refresh_token\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"access_token\",\"expires_in\",\"userId\"]}")
    }
}
impl Agent for AuthAuthSocialNetworkAuthReturns {
    fn topic() -> &'static str {
        "auth_auth_socialNetworkAuth"
    }
    fn method() -> &'static str {
        "auth_socialNetworkAuth"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
