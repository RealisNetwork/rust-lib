// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserMembershipInfoParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
}
impl Schema for CatsAndDragonsWrapperGetUserMembershipInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for CatsAndDragonsWrapperGetUserMembershipInfoParams {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getUserMembershipInfo"
    }
    fn method() -> &'static str {
        "wrapper_getUserMembershipInfo"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserMembershipInfoReturns {
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(rename = "statusPurchaseAppId")]
    pub status_purchase_app_id: f64,
    #[serde(rename = "balance", deserialize_with = "deserialize_to_string")]
    pub balance: String,
    #[serde(rename = "membershipId")]
    pub membership_id: f64,
    #[serde(rename = "multiplier")]
    pub multiplier: f64,
}
impl Schema for CatsAndDragonsWrapperGetUserMembershipInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isAuthorized\":{\"type\":\"boolean\"},\"statusPurchaseAppId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"balance\":{\"type\":\"string\"},\"membershipId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"multiplier\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"statusPurchaseAppId\",\"membershipId\",\"multiplier\",\"balance\",\"isAuthorized\"]}")
    }
}
impl Agent for CatsAndDragonsWrapperGetUserMembershipInfoReturns {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getUserMembershipInfo"
    }
    fn method() -> &'static str {
        "wrapper_getUserMembershipInfo"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
