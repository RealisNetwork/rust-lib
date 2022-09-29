// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsAndDragonsWrapperGetUserInfoParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appId\"]}") . unwrap ()
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
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(rename = "hasReferrer")]
    pub has_referrer: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isNewProfile")]
    pub is_new_profile: bool,
    #[serde(rename = "refCode")]
    pub ref_code: String,
    #[serde(rename = "refLink")]
    pub ref_link: String,
}
impl Schema for CatsAndDragonsWrapperGetUserInfoReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"email\":{\"type\":\"string\"},\"isAuthorized\":{\"type\":\"boolean\"},\"hasReferrer\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"isNewProfile\":{\"type\":\"boolean\"},\"refCode\":{\"type\":\"string\"},\"refLink\":{\"type\":\"string\"}},\"required\":[\"email\",\"isAuthorized\",\"refLink\",\"refCode\",\"hasReferrer\",\"nickname\",\"isNewProfile\"]}")
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
