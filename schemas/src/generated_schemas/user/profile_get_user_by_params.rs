// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetUserByParamsParams {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "ethWallet")]
    pub eth_wallet: Option<String>,
    #[serde(rename = "GPA")]
    pub gpa: Option<String>,
    #[serde(rename = "lisWallet")]
    pub lis_wallet: Option<String>,
    #[serde(rename = "supportId")]
    pub support_id: Option<String>,
}
impl Schema for UserProfileGetUserByParamsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"ethWallet\":{\"type\":\"string\"},\"GPA\":{\"type\":\"string\"},\"lisWallet\":{\"type\":\"string\"},\"supportId\":{\"type\":\"string\"}},\"required\":null}")
    }
}
impl Agent for UserProfileGetUserByParamsParams {
    fn topic() -> &'static str {
        "user_profile_getUserByParams"
    }
    fn method() -> &'static str {
        "profile_getUserByParams"
    }
    fn agent() -> &'static str {
        "user"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileGetUserByParamsReturns(pub String);
impl Schema for UserProfileGetUserByParamsReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for UserProfileGetUserByParamsReturns {
    fn topic() -> &'static str {
        "user_profile_getUserByParams"
    }
    fn method() -> &'static str {
        "profile_getUserByParams"
    }
    fn agent() -> &'static str {
        "user"
    }
}
