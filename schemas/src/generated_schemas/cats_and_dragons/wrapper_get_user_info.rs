// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoParams {
    #[serde(rename = "appId")]
    pub app_id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetUserInfoReturns {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isAuthorized")]
    pub is_authorized: bool,
    #[serde(rename = "refCode")]
    pub ref_code: String,
    #[serde(rename = "refLink")]
    pub ref_link: String,
    #[serde(rename = "hasReferrer")]
    pub has_referrer: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isNewProfile")]
    pub is_new_profile: bool,
}
