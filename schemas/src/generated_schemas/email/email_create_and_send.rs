// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailEmailCreateAndSendParams {
    #[serde(rename = "body", deserialize_with = "deserialize_to_string")]
    pub body: String,
    #[serde(rename = "isResend")]
    pub is_resend: Option<bool>,
    #[serde(rename = "receivers")]
    pub receivers: Vec<String>,
    #[serde(rename = "timeToSend")]
    pub time_to_send: f64,
    #[serde(rename = "subject", deserialize_with = "deserialize_to_string")]
    pub subject: String,
}
impl Schema for EmailEmailCreateAndSendParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"body\":{\"type\":\"string\"},\"isResend\":{\"type\":\"boolean\"},\"receivers\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"timeToSend\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"subject\":{\"type\":\"string\"}},\"required\":[\"body\",\"receivers\",\"timeToSend\",\"subject\"]}") . unwrap ()
    }
}
impl Agent for EmailEmailCreateAndSendParams {
    fn topic() -> &'static str {
        "email_email_createAndSend"
    }
    fn method() -> &'static str {
        "email_createAndSend"
    }
    fn agent() -> &'static str {
        "email"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
impl<'de> Deserialize<'de> for EmailEmailCreateAndSendReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(EmailEmailCreateAndSendReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct EmailEmailCreateAndSendReturns;
impl Schema for EmailEmailCreateAndSendReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for EmailEmailCreateAndSendReturns {
    fn topic() -> &'static str {
        "email_email_createAndSend"
    }
    fn method() -> &'static str {
        "email_createAndSend"
    }
    fn agent() -> &'static str {
        "email"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
