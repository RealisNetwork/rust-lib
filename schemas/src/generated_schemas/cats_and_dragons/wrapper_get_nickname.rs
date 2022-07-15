// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetNicknameParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetNicknameReturns {
    #[serde(rename = "nickname")]
    pub nickname: String,
}
