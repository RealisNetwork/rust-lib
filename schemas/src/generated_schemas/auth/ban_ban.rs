// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBanBanParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AuthBanBanParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for AuthBanBanParams {
    fn topic() -> &'static str {
        "auth_ban_ban"
    }
    fn method() -> &'static str {
        "ban_ban"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBanBanReturns(pub bool);
impl Schema for AuthBanBanReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthBanBanReturns {
    fn topic() -> &'static str {
        "auth_ban_ban"
    }
    fn method() -> &'static str {
        "ban_ban"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
