// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronCronGetParams {
    #[serde(rename = "id")]
    pub id: f64,
}
impl Schema for CronCronGetParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}") . unwrap ()
    }
}
impl Agent for CronCronGetParams {
    fn topic() -> &'static str {
        "cron_cron_get"
    }
    fn method() -> &'static str {
        "cron_get"
    }
    fn agent() -> &'static str {
        "cron"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronCronGetReturns {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "howOften")]
    pub how_often: f64,
    #[serde(rename = "startsAt")]
    pub starts_at: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
impl Schema for CronCronGetReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"key\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"howOften\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startsAt\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"key\",\"howOften\",\"startsAt\",\"createdAt\",\"updatedAt\"]}")
    }
}
impl Agent for CronCronGetReturns {
    fn topic() -> &'static str {
        "cron_cron_get"
    }
    fn method() -> &'static str {
        "cron_get"
    }
    fn agent() -> &'static str {
        "cron"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
