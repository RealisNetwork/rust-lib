// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
impl Schema for CatsAndDragonsWrapperGetUserInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"appId\"]}")
    }
}
impl Agent for CatsAndDragonsWrapperGetUserInfoParams {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getUserInfo"
    }
    fn method() -> &'static str {
        "wrapper_getUserInfo"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoReturns {
    #[serde(rename = "refLink")]
    pub ref_link: String,
    #[serde(rename = "isNewProfile")]
    pub is_new_profile: bool,
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "refCode")]
    pub ref_code: String,
    #[serde(rename = "hasReferrer")]
    pub has_referrer: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
}
impl Schema for CatsAndDragonsWrapperGetUserInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"refLink\":{\"type\":\"string\"},\"isNewProfile\":{\"type\":\"boolean\"},\"isAuthorized\":{\"type\":\"boolean\"},\"email\":{\"type\":\"string\"},\"refCode\":{\"type\":\"string\"},\"hasReferrer\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"}},\"required\":[\"email\",\"isAuthorized\",\"refLink\",\"refCode\",\"hasReferrer\",\"nickname\",\"isNewProfile\"]}")
    }
}
impl Agent for CatsAndDragonsWrapperGetUserInfoReturns {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getUserInfo"
    }
    fn method() -> &'static str {
        "wrapper_getUserInfo"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
}
