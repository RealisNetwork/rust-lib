// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusMembershipChangeMembershipGameParams {
    #[serde(rename = "newAppId")]
    pub new_app_id: i64,
    #[serde(rename = "id")]
    pub id: i64,
}
pub type StatusMembershipChangeMembershipGameReturns = bool;
