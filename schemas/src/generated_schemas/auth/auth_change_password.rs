// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAuthChangePasswordParams {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}
impl Schema for AuthAuthChangePasswordParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type AuthAuthChangePasswordReturns = bool;
