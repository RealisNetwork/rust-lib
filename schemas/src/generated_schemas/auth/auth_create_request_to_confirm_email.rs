// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthCreateRequestToConfirmEmailParams {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "referralCode")]
    pub referral_code: Option<String>,
    #[serde(rename = "deviceId")]
    pub device_id: Option<String>,
    #[serde(rename = "isResend")]
    pub is_resend: Option<bool>,
}
impl Schema for AuthAuthCreateRequestToConfirmEmailParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\",\"pattern\":\"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\\\.[a-zA-Z0-9-.]+$\"},\"referralCode\":{\"type\":\"string\"},\"deviceId\":{\"type\":\"string\"},\"isResend\":{\"type\":\"boolean\"}},\"required\":[\"email\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthCreateRequestToConfirmEmailParams {
    fn topic() -> &'static str {
        "auth_auth_createRequestToConfirmEmail"
    }
    fn method() -> &'static str {
        "auth_createRequestToConfirmEmail"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthCreateRequestToConfirmEmailReturns(pub String);
impl Schema for AuthAuthCreateRequestToConfirmEmailReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for AuthAuthCreateRequestToConfirmEmailReturns {
    fn topic() -> &'static str {
        "auth_auth_createRequestToConfirmEmail"
    }
    fn method() -> &'static str {
        "auth_createRequestToConfirmEmail"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
