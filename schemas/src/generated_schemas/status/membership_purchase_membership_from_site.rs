// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipPurchaseMembershipFromSiteParams {
    #[serde(rename = "membershipId")]
    pub membership_id: i64,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
impl Schema for StatusMembershipPurchaseMembershipFromSiteParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"membershipId\",\"appId\"]}")
    }
}
impl Agent for StatusMembershipPurchaseMembershipFromSiteParams {
    fn topic() -> &'static str {
        "status_membership_purchaseMembershipFromSite"
    }
    fn method() -> &'static str {
        "membership_purchaseMembershipFromSite"
    }
    fn agent() -> &'static str {
        "status"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipPurchaseMembershipFromSiteReturns(pub bool);
impl Schema for StatusMembershipPurchaseMembershipFromSiteReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for StatusMembershipPurchaseMembershipFromSiteReturns {
    fn topic() -> &'static str {
        "status_membership_purchaseMembershipFromSite"
    }
    fn method() -> &'static str {
        "membership_purchaseMembershipFromSite"
    }
    fn agent() -> &'static str {
        "status"
    }
}
