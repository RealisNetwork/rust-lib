// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthResentConfirmationMailParams {
    #[serde(rename = "email")]
    pub email: String,
}
impl Schema for AuthAuthResentConfirmationMailParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"}},\"required\":[\"email\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthResentConfirmationMailParams {
    fn topic() -> &'static str {
        "auth_auth_resentConfirmationMail"
    }
    fn method() -> &'static str {
        "auth_resentConfirmationMail"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthResentConfirmationMailReturns(pub String);
impl Schema for AuthAuthResentConfirmationMailReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for AuthAuthResentConfirmationMailReturns {
    fn topic() -> &'static str {
        "auth_auth_resentConfirmationMail"
    }
    fn method() -> &'static str {
        "auth_resentConfirmationMail"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
