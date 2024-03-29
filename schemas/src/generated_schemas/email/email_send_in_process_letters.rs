// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for EmailEmailSendInProcessLettersParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(EmailEmailSendInProcessLettersParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailSendInProcessLettersParams;
impl Schema for EmailEmailSendInProcessLettersParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for EmailEmailSendInProcessLettersParams {
    fn topic() -> &'static str {
        "email_email_sendInProcessLetters"
    }
    fn method() -> &'static str {
        "email_sendInProcessLetters"
    }
    fn agent() -> &'static str {
        "email"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for EmailEmailSendInProcessLettersReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(EmailEmailSendInProcessLettersReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailSendInProcessLettersReturns;
impl Schema for EmailEmailSendInProcessLettersReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for EmailEmailSendInProcessLettersReturns {
    fn topic() -> &'static str {
        "email_email_sendInProcessLetters"
    }
    fn method() -> &'static str {
        "email_sendInProcessLetters"
    }
    fn agent() -> &'static str {
        "email"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
