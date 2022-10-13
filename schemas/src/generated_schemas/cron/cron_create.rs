// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronCronCreateParams {
    #[serde(rename = "howOften")]
    pub how_often: f64,
    #[serde(rename = "startsAt")]
    pub starts_at: f64,
    #[serde(rename = "key")]
    pub key: String,
}
impl Schema for CronCronCreateParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"howOften\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startsAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"key\":{\"type\":\"string\"}},\"required\":[\"howOften\",\"startsAt\",\"key\"]}") . unwrap ()
    }
}
impl Agent for CronCronCreateParams {
    fn topic() -> &'static str {
        "cron_cron_create"
    }
    fn method() -> &'static str {
        "cron_create"
    }
    fn agent() -> &'static str {
        "cron"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronCronCreateReturns(pub bool);
impl Schema for CronCronCreateReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for CronCronCreateReturns {
    fn topic() -> &'static str {
        "cron_cron_create"
    }
    fn method() -> &'static str {
        "cron_create"
    }
    fn agent() -> &'static str {
        "cron"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
