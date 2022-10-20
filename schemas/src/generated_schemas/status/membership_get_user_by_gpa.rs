// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetUserByGpaParams {
    #[serde(rename = "gpa", deserialize_with = "deserialize_to_string")]
    pub gpa: String,
}
impl Schema for StatusMembershipGetUserByGpaParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"gpa\":{\"type\":\"string\"}},\"required\":[\"gpa\"]}") . unwrap ()
    }
}
impl Agent for StatusMembershipGetUserByGpaParams {
    fn topic() -> &'static str {
        "status_membership_getUserByGPA"
    }
    fn method() -> &'static str {
        "membership_getUserByGPA"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMembershipGetUserByGpaReturns(pub String);
impl Schema for StatusMembershipGetUserByGpaReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"string\"}")
    }
}
impl Agent for StatusMembershipGetUserByGpaReturns {
    fn topic() -> &'static str {
        "status_membership_getUserByGPA"
    }
    fn method() -> &'static str {
        "membership_getUserByGPA"
    }
    fn agent() -> &'static str {
        "status"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
