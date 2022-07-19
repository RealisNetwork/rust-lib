// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthAssignProviderAccountToDeviceIdParams {
    #[serde(rename = "token")]
    pub token: String,
}
impl Schema for AuthAuthAssignProviderAccountToDeviceIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"token\":{\"type\":\"string\"}},\"required\":[\"token\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthAssignProviderAccountToDeviceIdReturns(bool);
impl Schema for AuthAuthAssignProviderAccountToDeviceIdReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}