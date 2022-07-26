// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipAddAwardMembershipParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: i64,
    #[serde(rename = "days")]
    pub days: i64,
    #[serde(rename = "membershipId")]
    pub membership_id: i64,
}
impl Schema for StatusMembershipAddAwardMembershipParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"days\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"appId\",\"days\",\"membershipId\"]}")
    }
}
impl Agent for StatusMembershipAddAwardMembershipParams {
    fn topic() -> &'static str {
        "status_membership_addAwardMembership"
    }
    fn method() -> &'static str {
        "membership_addAwardMembership"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipAddAwardMembershipReturns(pub bool);
impl Schema for StatusMembershipAddAwardMembershipReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for StatusMembershipAddAwardMembershipReturns {
    fn topic() -> &'static str {
        "status_membership_addAwardMembership"
    }
    fn method() -> &'static str {
        "membership_addAwardMembership"
    }
    fn agent() -> &'static str {
        "status"
    }
}
