// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBanUnBanParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for AuthBanUnBanParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}") . unwrap ()
    }
}
impl Agent for AuthBanUnBanParams {
    fn topic() -> &'static str {
        "auth_ban_unBan"
    }
    fn method() -> &'static str {
        "ban_unBan"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBanUnBanReturns(pub bool);
impl Schema for AuthBanUnBanReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthBanUnBanReturns {
    fn topic() -> &'static str {
        "auth_ban_unBan"
    }
    fn method() -> &'static str {
        "ban_unBan"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
