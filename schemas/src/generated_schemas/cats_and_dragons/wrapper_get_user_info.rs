// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoParams {
    #[serde(rename = "appId")]
    pub app_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsAndDragonsWrapperGetUserInfoParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoReturns {
    #[serde(rename = "isNewProfile")]
    pub is_new_profile: bool,
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(rename = "refLink")]
    pub ref_link: String,
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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isNewProfile\":{\"type\":\"boolean\"},\"isAuthorized\":{\"type\":\"boolean\"},\"refLink\":{\"type\":\"string\"},\"email\":{\"type\":\"string\"},\"refCode\":{\"type\":\"string\"},\"hasReferrer\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"}},\"required\":[\"email\",\"isAuthorized\",\"refLink\",\"refCode\",\"hasReferrer\",\"nickname\",\"isNewProfile\"]}")
    }
}