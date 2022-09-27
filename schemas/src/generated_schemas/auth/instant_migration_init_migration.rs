// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInstantMigrationInitMigrationParams {
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
    #[serde(rename = "count")]
    pub count: f64,
    #[serde(rename = "startIndex")]
    pub start_index: f64,
    #[serde(rename = "interval")]
    pub interval: i32,
}
impl Schema for AuthInstantMigrationInitMigrationParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isPartial\":{\"type\":\"boolean\"},\"count\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"startIndex\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"interval\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"startIndex\",\"interval\",\"count\"]}")
    }
}
impl Agent for AuthInstantMigrationInitMigrationParams {
    fn topic() -> &'static str {
        "auth_instantMigration_initMigration"
    }
    fn method() -> &'static str {
        "instantMigration_initMigration"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInstantMigrationInitMigrationReturns(pub bool);
impl Schema for AuthInstantMigrationInitMigrationReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for AuthInstantMigrationInitMigrationReturns {
    fn topic() -> &'static str {
        "auth_instantMigration_initMigration"
    }
    fn method() -> &'static str {
        "instantMigration_initMigration"
    }
    fn agent() -> &'static str {
        "auth"
    }
}
