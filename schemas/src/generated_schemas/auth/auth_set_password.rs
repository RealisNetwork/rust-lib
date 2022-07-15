// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAuthSetPasswordParams {
    #[serde(rename = "providerId")]
    pub provider_id: String,
    #[serde(rename = "passwordHash")]
    pub password_hash: String,
    #[serde(rename = "appId")]
    pub app_id: i32,
    #[serde(rename = "password")]
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthAuthSetPasswordReturns {
    #[serde(rename = "expires_in")]
    pub expires_in: i32,
    #[serde(rename = "id_token")]
    pub id_token: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "refresh_expires_in")]
    pub refresh_expires_in: i32,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "access_token")]
    pub access_token: String,
}
