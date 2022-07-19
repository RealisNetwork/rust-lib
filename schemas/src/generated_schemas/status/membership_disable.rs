// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipDisableParams {
    #[serde(rename = "id")]
    pub id: i64,
}
impl Schema for StatusMembershipDisableParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\"]}")
    }
}
impl Agent for StatusMembershipDisableParams {
    fn topic() -> &'static str {
        "status_membership_disable"
    }
    fn method() -> &'static str {
        "membership_disable"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipDisableReturns(bool);
impl Schema for StatusMembershipDisableReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for StatusMembershipDisableReturns {
    fn topic() -> &'static str {
        "status_membership_disable"
    }
    fn method() -> &'static str {
        "membership_disable"
    }
    fn agent() -> &'static str {
        "status"
    }
}
