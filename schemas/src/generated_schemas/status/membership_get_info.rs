// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetInfoParams {
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
impl Schema for StatusMembershipGetInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for StatusMembershipGetInfoParams {
    fn topic() -> &'static str {
        "status_membership_getInfo"
    }
    fn method() -> &'static str {
        "membership_getInfo"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetInfoReturns {
    #[serde(rename = "multiplier")]
    pub multiplier: f64,
    #[serde(rename = "priceInLis")]
    pub price_in_lis: f64,
    #[serde(rename = "membership", deserialize_with = "deserialize_to_string")]
    pub membership: String,
    #[serde(rename = "price")]
    pub price: f64,
}
impl Schema for StatusMembershipGetInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"multiplier\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"priceInLis\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"membership\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"membership\",\"multiplier\",\"price\",\"priceInLis\"]}")
    }
}
impl Agent for StatusMembershipGetInfoReturns {
    fn topic() -> &'static str {
        "status_membership_getInfo"
    }
    fn method() -> &'static str {
        "membership_getInfo"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
