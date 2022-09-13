// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for EmailEmailSendNotSentLettersParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(EmailEmailSendNotSentLettersParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailSendNotSentLettersParams;
impl Schema for EmailEmailSendNotSentLettersParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for EmailEmailSendNotSentLettersParams {
    fn topic() -> &'static str {
        "email_email_sendNotSentLetters"
    }
    fn method() -> &'static str {
        "email_sendNotSentLetters"
    }
    fn agent() -> &'static str {
        "email"
    }
}
impl<'de> Deserialize<'de> for EmailEmailSendNotSentLettersReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(EmailEmailSendNotSentLettersReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailSendNotSentLettersReturns;
impl Schema for EmailEmailSendNotSentLettersReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for EmailEmailSendNotSentLettersReturns {
    fn topic() -> &'static str {
        "email_email_sendNotSentLetters"
    }
    fn method() -> &'static str {
        "email_sendNotSentLetters"
    }
    fn agent() -> &'static str {
        "email"
    }
}
