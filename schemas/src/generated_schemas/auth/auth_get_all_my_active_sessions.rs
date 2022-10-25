// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthGetAllMyActiveSessionsParams {
    #[serde(rename = "token", deserialize_with = "deserialize_to_string")]
    pub token: String,
}
impl Schema for AuthAuthGetAllMyActiveSessionsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":[\"token\"]}") . unwrap ()
    }
}
impl Agent for AuthAuthGetAllMyActiveSessionsParams {
    fn topic() -> &'static str {
        "auth_auth_getAllMyActiveSessions"
    }
    fn method() -> &'static str {
        "auth_getAllMyActiveSessions"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthGetAllMyActiveSessionsReturns {
    #[serde(rename = "lastAccess")]
    pub last_access: f64,
    #[serde(rename = "id", deserialize_with = "deserialize_to_string")]
    pub id: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "ipAddress", deserialize_with = "deserialize_to_string")]
    pub ip_address: String,
    #[serde(rename = "start")]
    pub start: f64,
    #[serde(rename = "username", deserialize_with = "deserialize_to_string")]
    pub username: String,
    #[serde(rename = "clients", deserialize_with = "deserialize_to_string")]
    pub clients: String,
}
impl Schema for AuthAuthGetAllMyActiveSessionsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"lastAccess\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"id\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"ipAddress\":{\"type\":\"string\"},\"start\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"username\":{\"type\":\"string\"},\"clients\":{\"type\":\"string\"}},\"required\":[\"id\",\"username\",\"userId\",\"ipAddress\",\"start\",\"lastAccess\",\"clients\"]}")
    }
}
impl Agent for AuthAuthGetAllMyActiveSessionsReturns {
    fn topic() -> &'static str {
        "auth_auth_getAllMyActiveSessions"
    }
    fn method() -> &'static str {
        "auth_getAllMyActiveSessions"
    }
    fn agent() -> &'static str {
        "auth"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
