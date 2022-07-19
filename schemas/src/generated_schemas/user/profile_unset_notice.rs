// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUnsetNoticeParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for UserProfileUnsetNoticeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileUnsetNoticeReturns(bool);
impl Schema for UserProfileUnsetNoticeReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
