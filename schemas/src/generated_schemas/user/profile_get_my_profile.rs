// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturnsBanParams {
    #[serde(rename = "reason")]
    pub reason: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "whoBanned")]
    pub who_banned: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "bannedUntil")]
    pub banned_until: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileGetMyProfileReturns {
    #[serde(rename = "suspicious")]
    pub suspicious: bool,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "isNicknameChanged")]
    pub is_nickname_changed: bool,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "isSubscribedToMailing")]
    pub is_subscribed_to_mailing: bool,
    #[serde(rename = "verified")]
    pub verified: bool,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "notice")]
    pub notice: String,
    #[serde(rename = "isBanned")]
    pub is_banned: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "ban")]
    pub ban: UserProfileGetMyProfileReturnsBanParams,
    #[serde(rename = "registeredAt")]
    pub registered_at: String,
}
