// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileChangeNicknameParams {
    #[serde(rename = "newNickname")]
    pub new_nickname: String,
}
impl Schema for UserProfileChangeNicknameParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type UserProfileChangeNicknameReturns = bool;
